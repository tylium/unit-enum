#![doc = include_str!("lib.md")]

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Error, Expr, Fields
          , Type, Variant};

/// Derives the `UnitEnum` trait for an enum.
///
/// This macro can be used on enums with unit variants (no fields) and optionally one "other" variant
/// that can hold arbitrary discriminant values.
///
/// # Attributes
/// - `#[repr(type)]`: Optional for regular enums, defaults to i32. Required when using an "other" variant.
/// - `#[unit_enum(other)]`: Marks a variant as the catch-all for undefined discriminant values.
///   The type of this variant must match the repr type.
///
/// # Requirements
/// - The enum must contain only unit variants, except for one optional "other" variant
/// - The "other" variant, if present, must:
///   - Be marked with `#[unit_enum(other)]`
///   - Have exactly one unnamed field matching the repr type
///   - Be the only variant with the "other" attribute
///   - Have a matching `#[repr(type)]` attribute
///
/// # Examples
///
/// Basic usage with unit variants (repr is optional):
/// ```rust
/// # use unit_enum::UnitEnum;
/// #[derive(UnitEnum)]
/// enum Example {
///     A,
///     B = 10,
///     C,
/// }
/// ```
///
/// Usage with explicit repr:
/// ```rust
/// # use unit_enum::UnitEnum;
/// #[derive(UnitEnum)]
/// #[repr(u16)]
/// enum Color {
///     Red = 10,
///     Green,
///     Blue = 45654,
/// }
/// ```
///
/// Usage with an "other" variant (repr required):
/// ```rust
/// # use unit_enum::UnitEnum;
/// #[derive(UnitEnum)]
/// #[repr(u16)]
/// enum Status {
///     Active = 1,
///     Inactive = 2,
///     #[unit_enum(other)]
///     Unknown(u16),  // type must match repr
/// }
/// ```
#[proc_macro_derive(UnitEnum, attributes(unit_enum))]
pub fn unit_enum_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    match validate_and_process(&ast) {
        Ok((discriminant_type, unit_variants, other_variant)) => {
            impl_unit_enum(&ast, &discriminant_type, &unit_variants, other_variant)
        }
        Err(e) => e.to_compile_error().into(),
    }
}

struct ValidationResult<'a> {
    unit_variants: Vec<&'a Variant>,
    other_variant: Option<(&'a Variant, Type)>,
}

fn validate_and_process(ast: &DeriveInput) -> Result<(Type, Vec<&Variant>, Option<(&Variant, Type)>), Error> {
    // Get discriminant type from #[repr] attribute
    let discriminant_type = get_discriminant_type(ast)?;

    let data_enum = match &ast.data {
        Data::Enum(data_enum) => data_enum,
        _ => return Err(Error::new_spanned(ast, "UnitEnum can only be derived for enums")),
    };

    let mut validation = ValidationResult {
        unit_variants: Vec::new(),
        other_variant: None,
    };

    // Validate each variant
    for variant in &data_enum.variants {
        match &variant.fields {
            Fields::Unit => {
                if has_unit_enum_attr(variant) {
                    return Err(Error::new_spanned(variant,
                                                  "Unit variants cannot have #[unit_enum] attributes"));
                }
                validation.unit_variants.push(variant);
            }
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                if has_unit_enum_other_attr(variant) {
                    if validation.other_variant.is_some() {
                        return Err(Error::new_spanned(variant,
                                                      "Multiple #[unit_enum(other)] variants found. Only one is allowed"));
                    }
                    validation.other_variant = Some((variant, fields.unnamed[0].ty.clone()));
                } else {
                    return Err(Error::new_spanned(variant,
                                                  "Non-unit variant must be marked with #[unit_enum(other)] to be used as the catch-all variant"));
                }
            }
            _ => return Err(Error::new_spanned(variant,
                                               "Invalid variant. UnitEnum only supports unit variants and a single tuple variant marked with #[unit_enum(other)]")),
        }
    }

    Ok((discriminant_type, validation.unit_variants, validation.other_variant))
}

fn get_discriminant_type(ast: &DeriveInput) -> Result<Type, Error> {
    ast.attrs.iter()
        .find(|attr| attr.path().is_ident("repr"))
        .map_or(Ok(syn::parse_quote!(i32)), |attr| {
            attr.parse_args::<Type>()
                .map_err(|_| Error::new_spanned(attr, "Invalid repr attribute"))
        })
}

fn has_unit_enum_attr(variant: &Variant) -> bool {
    variant.attrs.iter().any(|attr| attr.path().is_ident("unit_enum"))
}

fn has_unit_enum_other_attr(variant: &Variant) -> bool {
    variant.attrs.iter().any(|attr| {
        attr.path().is_ident("unit_enum") &&
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("other") {
                    Ok(())
                } else {
                    Err(meta.error("Invalid unit_enum attribute"))
                }
            }).is_ok()
    })
}

fn compute_discriminants(variants: &[&Variant]) -> Vec<Expr> {
    let mut discriminants = Vec::with_capacity(variants.len());
    let mut last_discriminant: Option<Expr> = None;

    for variant in variants {
        let discriminant = variant.discriminant.as_ref().map(|(_, expr)| expr.clone())
            .or_else(|| {
                last_discriminant.clone().map(|expr| syn::parse_quote! { #expr + 1 })
            })
            .unwrap_or_else(|| syn::parse_quote! { 0 });

        discriminants.push(discriminant.clone());
        last_discriminant = Some(discriminant);
    }

    discriminants
}

fn impl_unit_enum(
    ast: &DeriveInput,
    discriminant_type: &Type,
    unit_variants: &[&Variant],
    other_variant: Option<(&Variant, Type)>,
) -> TokenStream {
    let name = &ast.ident;
    let num_variants = unit_variants.len();
    let discriminants = compute_discriminants(unit_variants);

    let name_impl = generate_name_impl(name, unit_variants, &other_variant);
    let ordinal_impl = generate_ordinal_impl(name, unit_variants, &other_variant, num_variants);
    let from_ordinal_impl = generate_from_ordinal_impl(name, unit_variants);
    let discriminant_impl = generate_discriminant_impl(name, unit_variants, &other_variant, discriminant_type, &discriminants);
    let from_discriminant_impl = generate_from_discriminant_impl(name, unit_variants, &other_variant, discriminant_type, &discriminants);
    let values_impl = generate_values_impl(name, unit_variants, &discriminants, &other_variant);

    quote! {
        impl #name {
            #name_impl

            #ordinal_impl

            #from_ordinal_impl

            #discriminant_impl

            #from_discriminant_impl

            /// Returns the total number of unit variants in the enum (excluding the "other" variant if present).
            ///
            /// # Examples
            ///
            /// ```ignore
            /// # use unit_enum::UnitEnum;
            /// #[derive(UnitEnum)]
            /// enum Example {
            ///     A,
            ///     B,
            ///     #[unit_enum(other)]
            ///     Other(i32),
            /// }
            ///
            /// assert_eq!(Example::len(), 2);
            /// ```
            pub fn len() -> usize {
                #num_variants
            }

            #values_impl
        }
    }.into()
}

fn generate_name_impl(
    name: &syn::Ident,
    unit_variants: &[&Variant],
    other_variant: &Option<(&Variant, Type)>,
) -> proc_macro2::TokenStream {
    let unit_match_arms = unit_variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        quote! { #name::#variant_name => stringify!(#variant_name) }
    });

    let other_arm = other_variant.as_ref().map(|(variant, _)| {
        let variant_name = &variant.ident;
        quote! { #name::#variant_name(_) => stringify!(#variant_name) }
    });

    quote! {
        /// Returns the name of the enum variant as a string.
        ///
        /// # Examples
        ///
        /// ```ignore
        /// # use unit_enum::UnitEnum;
        /// #[derive(UnitEnum)]
        /// enum Example {
        ///     A,
        ///     B = 10,
        ///     C,
        /// }
        ///
        /// assert_eq!(Example::A.name(), "A");
        /// assert_eq!(Example::B.name(), "B");
        /// assert_eq!(Example::C.name(), "C");
        /// ```
        pub fn name(&self) -> &str {
            match self {
                #(#unit_match_arms,)*
                #other_arm
            }
        }
    }
}

fn generate_ordinal_impl(
    name: &syn::Ident,
    unit_variants: &[&Variant],
    other_variant: &Option<(&Variant, Type)>,
    num_variants: usize,
) -> proc_macro2::TokenStream {
    let unit_match_arms = unit_variants.iter().enumerate().map(|(index, variant)| {
        let variant_name = &variant.ident;
        quote! { #name::#variant_name => #index }
    });

    let other_arm = other_variant.as_ref().map(|(variant, _)| {
        let variant_name = &variant.ident;
        quote! { #name::#variant_name(_) => #num_variants }
    });

    quote! {
        /// Returns the zero-based ordinal of the enum variant.
        ///
        /// For enums with an "other" variant, it returns the position after all unit variants.
        ///
        /// # Examples
        ///
        /// ```ignore
        /// # use unit_enum::UnitEnum;
        /// #[derive(UnitEnum)]
        /// enum Example {
        ///     A,      // ordinal: 0
        ///     B = 10, // ordinal: 1
        ///     C,      // ordinal: 2
        /// }
        ///
        /// assert_eq!(Example::A.ordinal(), 0);
        /// assert_eq!(Example::B.ordinal(), 1);
        /// assert_eq!(Example::C.ordinal(), 2);
        /// ```
        pub fn ordinal(&self) -> usize {
            match self {
                #(#unit_match_arms,)*
                #other_arm
            }
        }
    }
}
fn generate_from_ordinal_impl(
    name: &syn::Ident,
    unit_variants: &[&Variant],
) -> proc_macro2::TokenStream {
    let match_arms = unit_variants.iter().enumerate().map(|(index, variant)| {
        let variant_name = &variant.ident;
        quote! { #index => Some(#name::#variant_name) }
    });

    quote! {
        /// Converts a zero-based ordinal to an enum variant, if possible.
        ///
        /// Returns `Some(variant)` if the ordinal corresponds to a unit variant,
        /// or `None` if the ordinal is out of range or would correspond to the "other" variant.
        ///
        /// # Examples
        ///
        /// ```ignore
        /// # use unit_enum::UnitEnum;
        /// # #[derive(Debug, PartialEq)]
        /// #[derive(UnitEnum)]
        /// enum Example {
        ///     A,
        ///     B,
        ///     #[unit_enum(other)]
        ///     Other(i32),
        /// }
        ///
        /// assert_eq!(Example::from_ordinal(0), Some(Example::A));
        /// assert_eq!(Example::from_ordinal(2), None); // Other variant
        /// assert_eq!(Example::from_ordinal(99), None); // Out of range
        /// ```
        pub fn from_ordinal(ord: usize) -> Option<Self> {
            match ord {
                #(#match_arms,)*
                _ => None
            }
        }
    }
}

fn generate_discriminant_impl(
    name: &syn::Ident,
    unit_variants: &[&Variant],
    other_variant: &Option<(&Variant, Type)>,
    discriminant_type: &Type,
    discriminants: &[Expr],
) -> proc_macro2::TokenStream {
    let unit_match_arms = unit_variants.iter().zip(discriminants).map(|(variant, discriminant)| {
        let variant_name = &variant.ident;
        quote! { #name::#variant_name => #discriminant }
    });

    let other_arm = other_variant.as_ref().map(|(variant, _)| {
        let variant_name = &variant.ident;
        quote! { #name::#variant_name(val) => *val }
    });

    quote! {
        /// Returns the discriminant value of the enum variant.
        ///
        /// For "other" variants, returns the contained value.
        ///
        /// # Examples
        ///
        /// ```ignore
        /// # use unit_enum::UnitEnum;
        /// #[derive(UnitEnum)]
        /// enum Example {
        ///     A,      // 0
        ///     B = 10, // 10
        ///     C,      // 11
        /// }
        ///
        /// assert_eq!(Example::A.discriminant(), 0);
        /// assert_eq!(Example::B.discriminant(), 10);
        /// assert_eq!(Example::C.discriminant(), 11);
        /// ```
         pub fn discriminant(&self) -> #discriminant_type {
            match self {
                #(#unit_match_arms,)*
                #other_arm
            }
        }
    }
}

fn generate_from_discriminant_impl(
    name: &syn::Ident,
    unit_variants: &[&Variant],
    other_variant: &Option<(&Variant, Type)>,
    discriminant_type: &Type,
    discriminants: &[Expr],
) -> proc_macro2::TokenStream {
    if let Some((other_variant, _)) = other_variant {
        let match_arms = unit_variants.iter().zip(discriminants).map(|(variant, discriminant)| {
            let variant_name = &variant.ident;
            quote! { x if x == #discriminant => #name::#variant_name }
        });

        let other_name = &other_variant.ident;
        quote! {
            /// Converts a discriminant value to an enum variant.
            ///
            /// For enums with an "other" variant, this will always return a value,
            /// using the "other" variant for undefined discriminants.
            ///
            /// # Examples
            ///
            /// ```ignore
            /// # use unit_enum::UnitEnum;
            /// #[derive(UnitEnum, PartialEq, Debug)]
            /// #[repr(u8)]
            /// enum Example {
            ///     A,      // 0
            ///     B = 10, // 10
            ///     #[unit_enum(other)]
            ///     Other(u8),
            /// }
            ///
            /// assert_eq!(Example::from_discriminant(0), Example::A);
            /// assert_eq!(Example::from_discriminant(10), Example::B);
            /// assert_eq!(Example::from_discriminant(42), Example::Other(42));
            /// ```
            pub fn from_discriminant(discr: #discriminant_type) -> Self {
                match discr {
                    #(#match_arms,)*
                    other => #name::#other_name(other)
                }
            }
        }
    } else {
        let match_arms = unit_variants.iter().zip(discriminants).map(|(variant, discriminant)| {
            let variant_name = &variant.ident;
            quote! { x if x == #discriminant => Some(#name::#variant_name) }
        });

        quote! {
            /// Converts a discriminant value to an enum variant, if possible.
            ///
            /// Returns `Some(variant)` if the discriminant corresponds to a defined variant,
            /// or `None` if the discriminant is undefined.
            ///
            /// # Examples
            ///
            /// ```ignore
            /// # use unit_enum::UnitEnum;
            /// #[derive(UnitEnum, PartialEq, Debug)]
            /// #[repr(u8)]
            /// enum Example {
            ///     A,      // 0
            ///     B = 10, // 10
            ///     C,      // 11
            /// }
            ///
            /// assert_eq!(Example::from_discriminant(0), Some(Example::A));
            /// assert_eq!(Example::from_discriminant(10), Some(Example::B));
            /// assert_eq!(Example::from_discriminant(42), None);
            /// ```
            pub fn from_discriminant(discr: #discriminant_type) -> Option<Self> {
                match discr {
                    #(#match_arms,)*
                    _ => None
                }
            }
        }
    }
}

fn generate_values_impl(
    name: &syn::Ident,
    unit_variants: &[&Variant],
    discriminants: &[Expr],
    _other_variant: &Option<(&Variant, Type)>,
) -> proc_macro2::TokenStream {
    // Create a vector of variant expressions paired with their discriminants
    let variant_exprs = unit_variants.iter().zip(discriminants).map(|(variant, _discriminant)| {
        let variant_name = &variant.ident;
        quote! {
            #name::#variant_name // The variant
        }
    });

    // Collect variants into a Vec to ensure consistent ordering
    quote! {
        /// Returns an iterator over all unit variants of the enum.
        ///
        /// Note: This does not include values from the "other" variant, if present.
        ///
        /// # Examples
        ///
        /// ```ignore
        /// # use unit_enum::UnitEnum;
        /// #[derive(UnitEnum, PartialEq, Debug)]
        /// enum Example {
        ///     A,
        ///     B,
        ///     #[unit_enum(other)]
        ///     Other(i32),
        /// }
        ///
        /// let values: Vec<_> = Example::values().collect();
        /// assert_eq!(values, vec![Example::A, Example::B]);
        /// ```
        pub fn values() -> impl Iterator<Item = Self> {
            vec![
                #(#variant_exprs),*
            ].into_iter()
        }
    }
}
