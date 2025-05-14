use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse_macro_input, Ident, Result, Token,
};

struct KeyValue {
    key: Ident,
    value: syn::Expr,
}

impl Parse for KeyValue {
    fn parse(input: ParseStream) -> Result<Self> {
        let key = input.parse()?;
        let _: Token![:] = input.parse()?;
        let value = input.parse()?;
        Ok(KeyValue { key, value })
    }
}

struct MetaBlock {
    _meta_token: Ident,
    _colon_token: Token![:],
    _brace_token: syn::token::Brace,
    fields: Punctuated<KeyValue, Token![,]>,
}

impl Parse for MetaBlock {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(MetaBlock {
            _meta_token: input.parse()?,
            _colon_token: input.parse()?,
            _brace_token: braced!(content in input),
            fields: content.parse_terminated(KeyValue::parse, Token![,])?,
        })
    }
}

struct AssetDefInput {
    _name_token: Ident,
    _name_colon: Token![:],
    name: Ident,
    _comma: Token![,],
    meta: MetaBlock,
}

impl Parse for AssetDefInput {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(AssetDefInput {
            _name_token: input.parse()?,
            _name_colon: input.parse()?,
            name: input.parse()?,
            _comma: input.parse()?,
            meta: input.parse()?,
        })
    }
}

struct AssetInput {
    _name_token: Ident,
    _name_colon: Token![:],
    name: Ident,
    _comma1: Token![,],
    _base_token: Ident,
    _base_colon: Token![:],
    base: Ident,
    _comma2: Token![,],
    meta: MetaBlock,
}

impl Parse for AssetInput {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(AssetInput {
            _name_token: input.parse()?,
            _name_colon: input.parse()?,
            name: input.parse()?,
            _comma1: input.parse()?,
            _base_token: input.parse()?,
            _base_colon: input.parse()?,
            base: input.parse()?,
            _comma2: input.parse()?,
            meta: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn asset_def(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as AssetDefInput);

    let asset_name = &input.name;
    let meta_struct_name = format_ident!("{}Meta", asset_name);

    let meta_fields = input.meta.fields.iter().map(|field| {
        let field_name = &field.key;
        let field_type = &field.value;

        quote! {
            pub #field_name: #field_type
        }
    });

    let expanded = quote! {
        #[derive(Debug, Clone, bincode::Encode, bincode::Decode)]
        pub struct #meta_struct_name {
            #(#meta_fields,)*
        }

        #[derive(Debug, Clone, bincode::Encode, bincode::Decode)]
        pub struct #asset_name {
            pub id: String,
            pub name: String,
            pub path: Option<String>,
            pub meta: #meta_struct_name,
            pub data: Vec<u8>,
        }

        impl #asset_name {
            pub fn new(name: impl Into<String>, meta: #meta_struct_name, data: Vec<u8>) -> Self {
                Self {
                    id: uuid::Uuid::new_v4().to_string(),
                    name: name.into(),
                    path: None,
                    meta,
                    data,
                }
            }

            pub fn with_path(mut self, path: impl Into<String>) -> Self {
                self.path = Some(path.into());
                self
            }

            pub fn serialize(&self) -> Result<Vec<u8>, bincode::error::EncodeError> {
                bincode::encode_to_vec(self, bincode::config::standard())
            }

            pub fn deserialize(bytes: &[u8]) -> Result<Self, bincode::error::DecodeError> {
                let (data, _size) = bincode::decode_from_slice(bytes, bincode::config::standard())?;
                Ok(data)
            }

            pub fn from_file(path: impl AsRef<std::path::Path>) -> Result<Self, std::io::Error> {
                let data = std::fs::read(path.as_ref())?;
                Self::deserialize(&data).map_err(|e| {
                    std::io::Error::new(std::io::ErrorKind::InvalidData, e)
                })
            }

            pub fn save_to_file(&self, path: impl AsRef<std::path::Path>) -> Result<(), std::io::Error> {
                let data = self.serialize().map_err(|e| {
                    std::io::Error::new(std::io::ErrorKind::InvalidData, e)
                })?;
                std::fs::write(path, data)
            }
        }

        impl AssetData for #asset_name {
            fn asset_type() -> AssetType {
                AssetType(std::any::TypeId::of::<#asset_name>())
            }

            fn from_bytes(bytes: &[u8], _config: &AssetConfig) -> Result<Self, AssetError> {
                Self::deserialize(bytes).map_err(|e| AssetError::InvalidFormat(e.to_string()))
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn asset(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as AssetInput);

    let asset_name = &input.name;
    let base_type = &input.base;
    let meta_struct_name = format_ident!("{}Meta", base_type);

    let meta_initializers = input.meta.fields.iter().map(|field| {
        let field_name = &field.key;
        let field_value = &field.value;

        quote! {
            #field_name: #field_value
        }
    });

    let expanded = quote! {
        lazy_static::lazy_static! {
            pub static ref #asset_name: #base_type = {
                let meta = #meta_struct_name {
                    #(#meta_initializers,)*
                };

                #base_type::new(
                    stringify!(#asset_name),
                    meta,
                    Vec::new()
                )
            };
        }
    };

    TokenStream::from(expanded)
}

struct AssetFileEntry {
    name: Ident,
    _colon: Token![:],
    base: Ident,
    _brace_token: syn::token::Brace,
    fields: Punctuated<KeyValue, Token![,]>,
}

impl Parse for AssetFileEntry {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let _colon: Token![:] = input.parse()?;
        let base: Ident = input.parse()?;
        let content;
        let _brace_token = braced!(content in input);
        let fields = content.parse_terminated(KeyValue::parse, Token![,])?;

        Ok(Self {
            name,
            _colon,
            base,
            _brace_token,
            fields,
        })
    }
}

struct AssetFileBlock {
    name: Ident,
    _comma: Token![,],
    _assets_token: Ident,
    _colon: Token![:],
    _brace_token: syn::token::Brace,
    entries: Punctuated<AssetFileEntry, Token![,]>,
}

impl Parse for AssetFileBlock {
    fn parse(input: ParseStream) -> Result<Self> {
        let _name_token: Ident = input.parse()?;
        let _color_token: Token![:] = input.parse()?;
        let name: Ident = input.parse()?;
        let comma: Token![,] = input.parse()?;
        let assets_token: Ident = input.parse()?;
        let colon: Token![:] = input.parse()?;

        let content;
        let brace_token = braced!(content in input);
        let entries = content.parse_terminated(AssetFileEntry::parse, Token![,])?;

        Ok(Self {
            name,
            _comma: comma,
            _assets_token: assets_token,
            _colon: colon,
            _brace_token: brace_token,
            entries,
        })
    }
}

#[proc_macro]
pub fn asset_file(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as AssetFileBlock);

    let global_name = &input.name;
    let first_entry = input.entries.first().expect("Expected at least one asset");
    let base_type = &first_entry.base;

    let asset_exprs = input.entries.iter().map(|entry| {
        let asset_name = &entry.name;
        let meta_type = format_ident!("{}Meta", entry.base); // still use specific meta type per asset

        let meta_fields = entry.fields.iter().map(|kv| {
            let key = &kv.key;
            let val = &kv.value;
            quote! { #key: #val }
        });

        quote! {
            {
                let meta = #meta_type {
                    #(#meta_fields,)*
                };
                #base_type::new(stringify!(#asset_name), meta, Vec::new())
            }
        }
    });

    let expanded = quote! {
        lazy_static::lazy_static! {
            pub static ref #global_name: Vec<#base_type> = vec![
                #(#asset_exprs),*
            ];
        }
    };

    TokenStream::from(expanded)
}
