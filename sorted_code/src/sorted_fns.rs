use super::aux::Items;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Error, Item, parse_macro_input};

pub fn sorted_fns_impl(tokens: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(tokens as Items);
    let mut prev_fn_name = None::<String>;
    let mut is_checking = true;

    for item in input.items.iter_mut() {
        if let Item::Fn(fn_item) = item {
            let sort_start = fn_item
                .attrs
                .iter()
                .any(|attr| attr.path().is_ident("sort_start"));
            let sort_end = fn_item
                .attrs
                .iter()
                .any(|attr| attr.path().is_ident("sort_end"));

            fn_item.attrs.retain(|attr| {
                !attr.path().is_ident("sort_start") && !attr.path().is_ident("sort_end")
            });

            if sort_end {
                is_checking = false;
            }

            if sort_start {
                is_checking = true;
                prev_fn_name = None;
            }

            if is_checking {
                let fn_name = fn_item.sig.ident.to_string();

                if let Some(prev_fn_name) = prev_fn_name
                    && prev_fn_name > fn_name
                {
                    return Error::new_spanned(
                        &fn_item.sig.ident,
                        [
                            "Functions are not sorted: ",
                            &prev_fn_name,
                            " before ",
                            &fn_name,
                        ]
                        .concat(),
                    )
                    .to_compile_error()
                    .into();
                }

                prev_fn_name = Some(fn_name);
            }
        }
    }

    let items = input.items;

    quote! { #(#items)* }.into()
}
