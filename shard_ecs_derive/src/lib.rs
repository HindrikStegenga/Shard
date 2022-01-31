use proc_macro::*;
use syn::{DeriveInput, parse_macro_input};
use quote::quote;

#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let name = input.ident;
    let expanded = quote! {
        impl #impl_generics Component for #name #ty_generics #where_clause {
            const NAME: &'static str = stringify!(#name);
        }
    };
    proc_macro::TokenStream::from(expanded)
}