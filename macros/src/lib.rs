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
    let AssetDefInput {
        struct_name,
        fields,
    } = syn::parse_macro_input!(input as AssetDefInput);

    let name_ident = syn::Ident::new("name", proc_macro2::Span::call_site());
    let string_type: syn::Type = syn::parse_quote!(String);

    let field_names: Vec<syn::Ident> = std::iter::once(name_ident.clone())
        .chain(fields.iter().map(|f| f.name.clone()))
        .collect();

    let field_types: Vec<syn::Type> = std::iter::once(string_type.clone())
        .chain(fields.iter().map(|f| f.ty.clone()))
        .collect();

    let expanded = quote! {
        #[derive(Debug, Clone, rasset::prelude::bincode::Encode, rasset::prelude::bincode::Decode)]
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

    TokenStream::from(expanded)
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
