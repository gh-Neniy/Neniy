use super::aux::Items;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Error, Item, parse_macro_input};

pub fn sorted_consts_impl(tokens: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(tokens as Items);
    let mut prev_const_name = None::<String>;
    let mut is_checking = true;

    for item in input.items.iter_mut() {
        if let Item::Const(const_item) = item {
            let sort_start = const_item
                .attrs
                .iter()
                .any(|attr| attr.path().is_ident("sort_start"));
            let sort_end = const_item
                .attrs
                .iter()
                .any(|attr| attr.path().is_ident("sort_end"));

            const_item.attrs.retain(|attr| {
                !attr.path().is_ident("sort_start") && !attr.path().is_ident("sort_end")
            });

            if sort_end {
                is_checking = false;
            }

            if sort_start {
                is_checking = true;
                prev_const_name = None;
            }

            if is_checking {
                let const_name = const_item.ident.to_string();

                if let Some(prev_const_name) = prev_const_name
                    && prev_const_name > const_name
                {
                    return Error::new_spanned(
                        &const_item.ident,
                        [
                            "Constants is not sorted: ",
                            &prev_const_name,
                            " before ",
                            &const_name,
                        ]
                        .concat(),
                    )
                    .to_compile_error()
                    .into();
                }

                prev_const_name = Some(const_name);
            }
        }
    }

    let items = input.items;

    quote! { #(#items)* }.into()
}
