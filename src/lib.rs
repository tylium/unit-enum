#![doc = include_str!("lib.md")]

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Expr, Fields, Type, punctuated::Punctuated, token::Comma, Variant};

/// Derives the `UnitEnum` trait for an enum.
///
/// This macro can only be used on enums with unit variants (no fields).
/// It respects the enum's `#[repr]` attribute for discriminant types.
///
/// # Example
///
/// ```rust
/// # use unit_enum::UnitEnum;
/// #[derive(UnitEnum)]
/// #[repr(u8)]
/// enum Example {
///     A,
///     B = 10,
///     C,
/// }
/// ```
#[proc_macro_derive(UnitEnum)]
pub fn unit_enum_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    // Look for #[repr(type)] attribute
    let discriminant_type = ast.attrs.iter()
        .find(|attr| attr.path().is_ident("repr"))
        .and_then(|attr| attr.parse_args::<Type>().ok())
        .unwrap_or_else(|| syn::parse_quote!(i32)); // default to i32

    impl_unit_enum(&ast, &discriminant_type)
}

fn impl_unit_enum(ast: &DeriveInput, discriminant_type: &Type) -> TokenStream {
    let name = &ast.ident;
    let data = match &ast.data {
        Data::Enum(data_enum) => data_enum,
        _ => panic!("UnitEnum is only defined for enums!"),
    };

    let variants = &data.variants;
    let num_variants = variants.len();

    let name_match_arms = variants.iter().enumerate().map(|(index, variant)| {
        let variant_name = &variant.ident;
        match &variant.fields {
            Fields::Unit => quote! { #name::#variant_name => stringify!(#variant_name) },
            _ => panic!("UnitEnum only supports unit variants (no fields)"),
        }
    });

    let ordinal_match_arms = variants.iter().enumerate().map(|(index, variant)| {
        let variant_name = &variant.ident;
        match &variant.fields {
            Fields::Unit => quote! { #name::#variant_name => #index },
            _ => panic!("UnitEnum only supports unit variants (no fields)"),
        }
    });

    let from_ordinal_match_arms = variants.iter().enumerate().map(|(index, variant)| {
        let variant_name = &variant.ident;
        quote! { #index => Some(#name::#variant_name) }
    });

    let values_arms = (0..num_variants).map(|index| {
        quote! { #name::from_ordinal(#index).unwrap() }
    });

    let (discriminant_match_arms, from_discriminant_match_arms) =
        generate_discriminant_arms(variants, name, discriminant_type);

    let gen = quote! {
        impl #name {
            /// Returns the name of the enum variant.
            pub fn name(&self) -> &str {
                match self {
                    #(#name_match_arms,)*
                }
            }

            /// Returns the zero-based ordinal of the enum variant.
            pub fn ordinal(&self) -> usize {
                match self {
                    #(#ordinal_match_arms,)*
                }
            }

            /// Converts a zero-based ordinal to an enum variant, if possible.
            ///
            /// Returns `None` if the ordinal is out of range.
            pub fn from_ordinal(ord: usize) -> Option<Self> {
                match ord {
                    #(#from_ordinal_match_arms,)*
                    _ => None,
                }
            }

            /// Returns the discriminant value of the enum variant.
            pub fn discriminant(&self) -> #discriminant_type {
                match self {
                    #(#discriminant_match_arms,)*
                }
            }

            /// Converts a discriminant value back to an enum variant, if possible.
            ///
            /// Returns `None` if the discriminant does not correspond to any variant.
            pub fn from_discriminant(discr: #discriminant_type) -> Option<Self> {
                match discr {
                    #(#from_discriminant_match_arms,)*
                    _ => None,
                }
            }

            /// Returns the total number of variants in the enum.
            pub fn len() -> usize {
                #num_variants
            }

            /// Returns an iterator over all variants of the enum.
            pub fn values() -> impl Iterator<Item = Self> {
                vec![#(#values_arms,)*].into_iter()
            }
        }
    };
    gen.into()
}

fn generate_discriminant_arms(
    variants: &Punctuated<Variant, Comma>,
    name: &syn::Ident,
    discriminant_type: &Type,
) -> (Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>) {
    let mut last_discriminant: Option<Expr> = None;
    let discriminant_match_arms = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let discriminant = variant.discriminant.as_ref().map(|(_, expr)| expr.clone())
            .or_else(|| last_discriminant.clone()).unwrap_or_else(|| syn::parse_quote! { 0 });
        last_discriminant = Some(syn::parse_quote! { #discriminant + 1 });

        quote! { #name::#variant_name => #discriminant as #discriminant_type }
    }).collect::<Vec<_>>();

    let from_discriminant_match_arms = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let discriminant = variant.discriminant.as_ref().map(|(_, expr)| expr.clone())
            .or_else(|| last_discriminant.clone()).unwrap_or_else(|| syn::parse_quote! { 0 });

        quote! { x if x == (#discriminant as #discriminant_type) => Some(#name::#variant_name) }
    }).collect::<Vec<_>>();

    (discriminant_match_arms, from_discriminant_match_arms)
}