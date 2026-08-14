use proc_macro::TokenStream;
use quote::quote;
use syn::{Error, ItemEnum, parse_macro_input};

pub fn sorted_enum_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut is_checking = true;
    let mut prev_variant_name: Option<String> = None;
    let mut input = parse_macro_input!(item as ItemEnum);

    for variant in input.variants.iter_mut() {
        let sort_start = variant
            .attrs
            .iter()
            .any(|attr| attr.path().is_ident("sort_start"));
        let sort_end = variant
            .attrs
            .iter()
            .any(|attr| attr.path().is_ident("sort_end"));

        variant.attrs.retain(|attr| {
            !attr.path().is_ident("sort_start") && !attr.path().is_ident("sort_end")
        });

        if sort_end {
            is_checking = false;
        }

        if sort_start {
            is_checking = true;
            prev_variant_name = None;
        }

        if is_checking {
            let variant_name = variant.ident.to_string();

            if let Some(prev_variant_name) = prev_variant_name
                && prev_variant_name > variant_name
            {
                return Error::new_spanned(
                    &variant.ident,
                    [
                        "Enum ",
                        &input.ident.to_string(),
                        " is not sorted: ",
                        &prev_variant_name,
                        " before ",
                        &variant_name,
                    ]
                    .concat(),
                )
                .to_compile_error()
                .into();
            }

            prev_variant_name = Some(variant_name);
        }
    }

    quote! { #input }.into()
}
