use proc_macro::TokenStream;
use quote::quote;
use syn::{Error, ImplItem, ItemImpl, Type, parse_macro_input};

pub fn sorted_methods_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemImpl);
    let mut prev_fn_name = None::<String>;
    let mut is_checking = true;

    for impl_item in input.items.iter_mut() {
        if let ImplItem::Fn(method_item) = impl_item {
            let sort_start = method_item
                .attrs
                .iter()
                .any(|attr| attr.path().is_ident("sort_start"));
            let sort_end = method_item
                .attrs
                .iter()
                .any(|attr| attr.path().is_ident("sort_end"));

            method_item.attrs.retain(|attr| {
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
                let fn_name = method_item.sig.ident.to_string();

                if let Some(prev_fn_name) = prev_fn_name
                    && prev_fn_name > fn_name
                {
                    let mut error_msg = "Methods".to_string();

                    if let Type::Path(type_path) = *input.self_ty {
                        error_msg.extend([
                            " of ",
                            &type_path.path.segments.last().unwrap().ident.to_string(),
                        ]);
                    }

                    error_msg.extend([" are not sorted: ", &prev_fn_name, " before ", &fn_name]);

                    return Error::new_spanned(&method_item.sig.ident, error_msg)
                        .to_compile_error()
                        .into();
                }

                prev_fn_name = Some(fn_name);
            }
        }
    }

    quote! { #input }.into()
}
