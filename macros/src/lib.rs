use proc_macro::TokenStream;
use quote::quote;
use syn::{
    braced,
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    token::Comma,
    FieldValue, Ident, Member, Token, Type,
};

struct AssetDefInput {
    struct_name: Ident,
    fields: Punctuated<FieldDef, Comma>,
}

struct FieldDef {
    name: Ident,
    ty: Type,
}

struct AssetDefsInput {
    defs: Punctuated<AssetDefInput, Token![,]>,
}

impl Parse for AssetDefsInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let defs = Punctuated::parse_terminated(input)?;
        Ok(AssetDefsInput { defs })
    }
}

impl Parse for FieldDef {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let ty: Type = input.parse()?;
        Ok(FieldDef { name, ty })
    }
}

impl Parse for AssetDefInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let struct_name: Ident = input.parse()?;
        let _colon: Token![:] = input.parse()?;

        let content;
        braced!(content in input);

        let fields = Punctuated::<FieldDef, Comma>::parse_terminated(&content)?;

        Ok(AssetDefInput {
            struct_name,
            fields,
        })
    }
}

#[proc_macro]
pub fn asset_def(input: TokenStream) -> TokenStream {
    let AssetDefsInput { defs } = syn::parse_macro_input!(input as AssetDefsInput);

    let mut expanded_tokens = proc_macro2::TokenStream::new();

    for def in defs {
        let struct_name = &def.struct_name;
        let fields = &def.fields;

        let name_ident = syn::Ident::new("name", proc_macro2::Span::call_site());
        let string_type: syn::Type = syn::parse_quote!(String);

        let field_names: Vec<syn::Ident> = std::iter::once(name_ident.clone())
            .chain(fields.iter().map(|f| f.name.clone()))
            .collect();

        let field_types: Vec<syn::Type> = std::iter::once(string_type.clone())
            .chain(fields.iter().map(|f| f.ty.clone()))
            .collect();

        let expanded = quote! {
            #[derive(Debug, Clone, bincode::Encode, bincode::Decode)]
            pub struct #struct_name {
                #(pub #field_names: #field_types),*
            }

            impl rasset::prelude::Asset for #struct_name {
                fn get_type(&self) -> rasset::prelude::Type {
                    rasset::prelude::Type(std::any::TypeId::of::<#struct_name>())
                }

                fn type_name(&self) -> &'static str {
                    std::any::type_name::<Self>()
                }

                fn as_any(&self) -> &dyn std::any::Any {
                    self
                }

                fn name(&self) -> String {
                    self.name.clone()
                }

                fn to_bytes(&self) -> Result<Vec<u8>, rasset::prelude::Error> {
                    bincode::encode_to_vec(self, bincode::config::standard())
                        .map_err(|e| rasset::prelude::Error::Serialization(format!("Failed to serialize {}: {}", stringify!(#struct_name), e)))
                }

                fn from_bytes(bytes: &[u8]) -> Result<Self, rasset::prelude::Error> {
                    bincode::decode_from_slice(bytes, bincode::config::standard())
                        .map_err(|e| rasset::prelude::Error::Deserialization(format!("Failed to deserialize {}: {}", stringify!(#struct_name), e)))
                        .map(|(asset, _)| asset)
                }
            }
        };

        expanded_tokens.extend(expanded);
    }

    TokenStream::from(expanded_tokens)
}

struct AssetInstance {
    name: Ident,
    ty: Type,
    fields: Punctuated<FieldValue, Comma>,
}

impl Parse for AssetInstance {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<syn::Token![:]>()?;

        let ty: Type = input.parse()?;

        let content;
        braced!(content in input);

        let fields = Punctuated::<FieldValue, Comma>::parse_terminated(&content)?;

        Ok(AssetInstance { name, ty, fields })
    }
}

struct AssetsInput {
    assets: Punctuated<AssetInstance, Comma>,
}

impl Parse for AssetsInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let assets = Punctuated::<AssetInstance, Comma>::parse_terminated(input)?;
        Ok(AssetsInput { assets })
    }
}

#[proc_macro]
pub fn assets(input: TokenStream) -> TokenStream {
    let AssetsInput { assets } = syn::parse_macro_input!(input as AssetsInput);

    let asset_inits = assets.iter().map(|asset| {
        let AssetInstance { name, ty, fields } = asset;
        let name_str = name.to_string();

        let mut field_inits = fields
            .iter()
            .map(|field| {
                let field_name = match &field.member {
                    Member::Named(ident) => ident,
                    _ => panic!("Expected named field"),
                };
                let expr = &field.expr;
                quote! { #field_name: #expr }
            })
            .collect::<Vec<_>>();

        field_inits.insert(0, quote! { name: #name_str.to_string() });

        quote! {
            {
                let mut asset = #ty {
                    #(#field_inits),*
                };
                asset.name = #name_str.to_string();
                asset
            }
        }
    });

    let expanded = quote! {
        pub fn compile_assets() -> Result<Vec<u8>, Error> {
            let mut compiler = rasset::prelude::Compiler::new();
            #(compiler.add_asset(Box::new(#asset_inits));)*
            Ok(compiler.compile()?.to_vec())
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn asset_file(input: TokenStream) -> TokenStream {
    let file_path_lit = syn::parse_macro_input!(input as syn::LitStr);
    let file_path = file_path_lit.value();

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let absolute_path = std::path::Path::new(&manifest_dir).join(&file_path);

    let contents = std::fs::read_to_string(&absolute_path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", absolute_path.display(), e));

    #[derive(Debug, serde::Deserialize)]
    struct AssetYaml {
        name: String,
        #[serde(rename = "type")]
        ty: String,
        metadata: std::collections::HashMap<String, serde_yaml::Value>,
    }

    let parsed: Vec<AssetYaml> = serde_yaml::from_str(&contents)
        .unwrap_or_else(|e| panic!("Failed to parse YAML {}: {}", file_path, e));

    let asset_tokens = parsed.into_iter().map(|asset| {
        let _name_ident = syn::Ident::new(&asset.name, proc_macro2::Span::call_site());
        let ty_ident = syn::Ident::new(&asset.ty, proc_macro2::Span::call_site());

        let field_inits = asset.metadata.iter().map(|(k, v)| {
            let key = k.as_str();
            let ident = syn::Ident::new(key, proc_macro2::Span::call_site());

            let expr = yaml_value_to_expr(v);
            quote! { #ident: #expr }
        });

        let name_string = &asset.name;
        quote! {
            {
                let mut asset = #ty_ident {
                    name: #name_string.to_string(),
                    #(#field_inits),*
                };
                asset
            }
        }
    });

    let expanded = quote! {
        pub fn compile_assets() -> Result<Vec<u8>, Error> {
            let mut compiler = rasset::prelude::Compiler::new();
            #(compiler.add_asset(Box::new(#asset_tokens));)*
            Ok(compiler.compile()?.to_vec())
        }
    };

    TokenStream::from(expanded)
}

fn yaml_value_to_expr(value: &serde_yaml::Value) -> proc_macro2::TokenStream {
    match value {
        serde_yaml::Value::Tagged(tagged) => match tagged.tag.to_string().as_str() {
            "!Rust" => {
                if let serde_yaml::Value::String(expr) = &tagged.value {
                    let tokens: proc_macro2::TokenStream =
                        expr.parse().expect("Invalid Rust expression");
                    quote! { #tokens }
                } else {
                    panic!("!!Rust must wrap a string");
                }
            }
            "!IncludeBytes" => {
                if let serde_yaml::Value::String(path) = &tagged.value {
                    quote! { include_bytes!(#path) }
                } else {
                    panic!("!IncludeBytes must wrap a string");
                }
            }
            "!IncludeStr" => {
                if let serde_yaml::Value::String(path) = &tagged.value {
                    quote! { include_str!(#path).to_string() }
                } else {
                    panic!("!IncludeString must wrap a string");
                }
            }
            "!IncludeVec" => {
                if let serde_yaml::Value::String(path) = &tagged.value {
                    quote! { include_bytes!(#path).to_vec() }
                } else {
                    panic!("!IncludeBytes must wrap a string");
                }
            }
            _ => yaml_value_to_expr(&tagged.value),
        },
        serde_yaml::Value::String(s) => quote! { #s.to_string() },
        serde_yaml::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                quote! { #i }
            } else if let Some(f) = n.as_f64() {
                quote! { #f }
            } else {
                panic!("Unsupported number type: {}", n);
            }
        }
        serde_yaml::Value::Sequence(seq) => {
            let elems: Vec<_> = seq.iter().map(yaml_value_to_expr).collect();
            if seq
                .iter()
                .all(|v| matches!(v, serde_yaml::Value::String(_)))
            {
                quote! { vec![#(#elems),*] }
            } else {
                quote! { (#(#elems),*) }
            }
        }
        serde_yaml::Value::Bool(b) => quote! { #b },
        serde_yaml::Value::Null => quote! { () },
        _ => panic!("Unsupported YAML value: {:?}", value),
    }
}
