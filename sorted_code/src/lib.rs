use proc_macro::TokenStream;

mod aux;
mod sorted_consts;
mod sorted_enum;
mod sorted_fns;
mod sorted_match;
mod sorted_methods;

#[proc_macro]
pub fn sorted_consts(tokens: TokenStream) -> TokenStream {
    sorted_consts::sorted_consts_impl(tokens)
}

#[proc_macro_attribute]
pub fn sorted_enum(attr: TokenStream, item: TokenStream) -> TokenStream {
    sorted_enum::sorted_enum_impl(attr, item)
}

#[proc_macro]
pub fn sorted_fns(tokens: TokenStream) -> TokenStream {
    sorted_fns::sorted_fns_impl(tokens)
}

#[proc_macro]
pub fn sorted_match(tokens: TokenStream) -> TokenStream {
    sorted_match::sorted_match_impl(tokens)
}

#[proc_macro_attribute]
pub fn sorted_methods(attr: TokenStream, item: TokenStream) -> TokenStream {
    sorted_methods::sorted_methods_impl(attr, item)
}
