use proc_macro::TokenStream;
use quote::quote;
use syn::{Error, ExprMatch, Pat, parse_macro_input};

pub fn sorted_match_impl(tokens: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(tokens as ExprMatch);
    let mut prev_arm_name = None::<String>;
    let mut is_checking = true;

    for arm in input.arms.iter_mut() {
        let sort_start = arm
            .attrs
            .iter()
            .any(|attr| attr.path().is_ident("sort_start"));
        let sort_end = arm
            .attrs
            .iter()
            .any(|attr| attr.path().is_ident("sort_end"));

        arm.attrs.retain(|attr| {
            !attr.path().is_ident("sort_start") && !attr.path().is_ident("sort_end")
        });

        if sort_end {
            is_checking = false;
        }

        if sort_start {
            is_checking = true;
            prev_arm_name = None;
        }

        if is_checking {
            let pattern = &arm.pat;

            if let Pat::Or(pattern_or) = pattern {
                let mut prev_case_name = None::<String>;

                for case in pattern_or.cases.iter() {
                    let case_name = quote!(#case).to_string();

                    if case_name != "_"
                        && let Some(prev_case_name) = prev_case_name
                        && prev_case_name > case_name
                    {
                        return Error::new_spanned(
                            pattern,
                            [
                                "Match sub-pattens are not sorted: ",
                                &prev_case_name,
                                " before ",
                                &case_name,
                            ]
                            .concat(),
                        )
                        .to_compile_error()
                        .into();
                    }

                    prev_case_name = Some(case_name);
                }
            }

            let arm_name = quote!(#pattern).to_string();

            if arm_name != "_"
                && let Some(prev_arm_name) = prev_arm_name
                && prev_arm_name > arm_name
            {
                return Error::new_spanned(
                    pattern,
                    [
                        "Match arms are not sorted: ",
                        &prev_arm_name,
                        " before ",
                        &arm_name,
                    ]
                    .concat(),
                )
                .to_compile_error()
                .into();
            }

            prev_arm_name = Some(arm_name);
        }
    }

    quote! { #input }.into()
}
