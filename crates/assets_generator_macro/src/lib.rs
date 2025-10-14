use std::{path::Path};

// lib.rs in your proc-macro crate
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::{Parse, ParseStream}, parse_macro_input, Ident, LitStr, Token};

use crate::utils::collect_assets;

mod utils;

// Define the input structure
struct EnumFromFileInput {
    enum_name: Ident,
    file_path: LitStr,
}

impl Parse for EnumFromFileInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let enum_name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let file_path: LitStr = input.parse()?;
        Ok(EnumFromFileInput { enum_name, file_path })
    }
}

#[proc_macro]
pub fn assets_enum(input: TokenStream) -> TokenStream {
    let EnumFromFileInput { enum_name, file_path } = parse_macro_input!(input as EnumFromFileInput);
    let mut base_path = Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();
    base_path.push(&file_path.value());
    println!("base_path {:?}", base_path);
    let variants = collect_assets(Path::new(&file_path.value())).unwrap_or_default();

    // Generate enum variants
    let variant_names: Vec<_> = variants.iter()
        .map(|(name, _)| {
            let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
            quote! { #ident }
        })
        .collect();

    // Generate match arms for trait implementation
    let match_arms: Vec<_> = variants.iter()
        .map(|(name, desc)| {
            let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
            quote! { Self::#ident => #desc }
        })
        .collect();

    // Generate the output
    let expanded = quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum #enum_name {
            #(#variant_names),*
        }

        impl AssetsTools for #enum_name {
            fn path(&self) -> &str {
                match self {
                    #(#match_arms),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
