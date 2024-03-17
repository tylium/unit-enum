#![doc = include_str!("lib.md")]

use proc_macro::TokenStream;

use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(UnitEnum)]
pub fn unit_enum_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    impl_unit_enum(&ast)
}

fn impl_unit_enum(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let data = match &ast.data {
        Data::Enum(data_enum) => data_enum,
        _ => panic!("UnitEnum is only defined for enums!"),
    };

    let variants = &data.variants;
    let num_variants = variants.len(); // Count the number of variants

    let ordinal_match_arms = variants.iter().enumerate().map(|(index, variant)| {
        let variant_name = &variant.ident;
        match &variant.fields {
            Fields::Unit => {
                quote! { #name::#variant_name => #index, }
            },
            _ => panic!("UnitEnum only supports unit variants (no fields)"),
        }
    });

    let from_ordinal_match_arms = variants.iter().enumerate().map(|(index, variant)| {
        let variant_name = &variant.ident;
        quote! { #index => Some(#name::#variant_name), }
    });

    let gen = quote! {
        impl #name {
            pub fn ordinal(&self) -> usize {
                match self {
                    #( #ordinal_match_arms )*
                }
            }

            pub fn from_ordinal(ord: usize) -> Option<Self> {
                match ord {
                    #( #from_ordinal_match_arms )*
                    _ => None,
                }
            }

            pub fn len() -> usize {
                #num_variants
            }
        }
    };
    gen.into()
}
