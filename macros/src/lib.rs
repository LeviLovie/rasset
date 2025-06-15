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
    struct Asset {
        name: String,
        #[serde(rename = "type")]
        ty: String,
        metadata: std::collections::BTreeMap<ron::Value, ron::Value>,
    }

    let parsed: Vec<Asset> = ron::from_str(&contents)
        .unwrap_or_else(|e| panic!("Failed parsing RON from {}: {}", file_path, e));

    let asset_tokens = parsed.into_iter().map(|asset| {
        let _name_ident = syn::Ident::new(&asset.name, proc_macro2::Span::call_site());
        let ty_ident = syn::Ident::new(&asset.ty, proc_macro2::Span::call_site());

        let field_inits = asset.metadata.iter().map(|(k, v)| {
            let key: String = k.clone().into_rust().expect("Key must be a string");
            let ident = syn::Ident::new(&key, proc_macro2::Span::call_site());

            let expr = value_to_expr(v);
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

fn value_to_expr(value: &ron::Value) -> proc_macro2::TokenStream {
    match value {
        ron::Value::Bool(b) => quote! { #b },
        ron::Value::Char(c) => quote! { #c },
        ron::Value::Map(map) => {
            let entries: Vec<_> = map
                .iter()
                .map(|(k, v)| {
                    let key = value_to_expr(k);
                    let value = value_to_expr(v);
                    quote! { (#key, #value) }
                })
                .collect();
            quote! { std::collections::HashMap::from([#(#entries),*]) }
        }
        ron::Value::Number(n) => match n {
            ron::Number::I8(i) => {
                let i = *i as i64;
                quote! { #i }
            }
            ron::Number::I16(i) => {
                let i = *i as i64;
                quote! { #i }
            }
            ron::Number::I32(i) => {
                let i = *i as i64;
                quote! { #i }
            }
            ron::Number::I64(i) => quote! { #i },
            ron::Number::U8(u) => {
                let u = *u as i64;
                quote! { #u }
            }
            ron::Number::U16(u) => {
                let u = *u as i64;
                quote! { #u }
            }
            ron::Number::U32(u) => {
                let u = *u as i64;
                quote! { #u }
            }
            ron::Number::U64(u) => {
                let u = *u as i64;
                quote! { #u }
            }
            ron::Number::F32(f) => {
                let f = f.0 as f64;
                quote! { #f.0 }
            }
            ron::Number::F64(f) => {
                let f = f.0;
                quote! { #f.0 }
            }
            ron::Number::__NonExhaustive(_) => {
                panic!("Unsupported RON number type");
            }
        },
        ron::Value::Option(Some(v)) => value_to_expr(v),
        ron::Value::Option(None) => quote! { None },
        ron::Value::String(s) => {
            if s.starts_with("!Rust ") {
                let expr_str = s.trim_start_matches("!Rust ");
                let tokens: proc_macro2::TokenStream =
                    expr_str.parse().expect("Invalid Rust expression");
                return quote! { #tokens };
            } else if s.starts_with("!IncludeBytes ") {
                let path = s.trim_start_matches("!IncludeBytes ");
                return quote! { include_bytes!(#path).to_vec() };
            } else if s.starts_with("!IncludeStr ") {
                let path = s.trim_start_matches("!IncludeStr ");
                return quote! { include_str!(#path).to_string() };
            } else if s.starts_with("!IncludeVec ") {
                let path = s.trim_start_matches("!IncludeVec ");
                return quote! { include_bytes!(#path).to_vec() };
            }
            quote! { #s.to_string() }
        }
        ron::Value::Seq(seq) => {
            let elements: Vec<_> = seq.iter().map(value_to_expr).collect();
            quote! { vec![#(#elements),*] }
        }
        ron::Value::Unit => quote! { () },
        _ => {
            panic!("Unsupported RON value type: {:?}", value);
        }
    }
}
