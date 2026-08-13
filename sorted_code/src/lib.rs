use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    {Error, ExprMatch, Item, ItemEnum, Result, parse_macro_input},
};

#[proc_macro_attribute]
pub fn sorted_enum(_attr: TokenStream, item: TokenStream) -> TokenStream {
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

struct Items {
    items: Vec<Item>,
}

impl Parse for Items {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut items = Vec::new();

        while !input.is_empty() {
            items.push(input.parse()?);
        }

        Ok(Items { items })
    }
}

#[proc_macro]
pub fn sorted_consts(tokens: TokenStream) -> TokenStream {
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

#[proc_macro]
pub fn sorted_match(tokens: TokenStream) -> TokenStream {
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
            let arm_name = quote!(#pattern).to_string();

            if arm_name != "_"
                && let Some(prev_arm_name) = prev_arm_name
                && prev_arm_name > arm_name
            {
                return Error::new_spanned(
                    pattern,
                    [
                        "Match arms is not sorted: ",
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
