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
            }
            _ => panic!("UnitEnum only supports unit variants (no fields)"),
        }
    });

    let from_ordinal_match_arms = variants.iter().enumerate().map(|(index, variant)| {
        let variant_name = &variant.ident;
        quote! { #index => Some(#name::#variant_name), }
    });

    let values_arms = (0..num_variants).map(|index| {
        quote! { #name::from_ordinal(#index).unwrap() }
    });

    let gen = quote! {
        impl #name {
            /// Returns the zero-based ordinal of the enum variant.
            pub fn ordinal(&self) -> usize {
                match self {
                    #( #ordinal_match_arms )*
                }
            }

            /// Converts a zero-based ordinal to an enum variant, if possible.
            ///
            /// Returns `None` if the ordinal is out of range.
             pub fn from_ordinal(ord: usize) -> Option<Self> {
                match ord {
                    #( #from_ordinal_match_arms )*
                    _ => None,
                }
            }

            /// Returns the total number of variants in the enum.
            pub fn len() -> usize {
                #num_variants
            }

            /// Returns an iterator over all variants of the enum.
            ///
            /// This method creates an iterator that yields each variant in
            /// definition order, starting from the first variant.
            pub fn values() -> impl Iterator<Item = Self> {
                vec![ #( #values_arms ),* ].into_iter()
            }
        }
    };
    gen.into()
}