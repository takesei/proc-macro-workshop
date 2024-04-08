use proc_macro::TokenStream;
use quote::quote;

// pub struct DeriveInput {
//    pub attrs: Vec<Attribute>,
//    pub vis: Visibility,
//    pub ident: Ident,
//    pub generics: Generics,
//    pub data: Data,
//}

#[proc_macro_derive(Builder)]
pub fn derive(_: TokenStream) -> TokenStream {
    let expanded = quote! {};
    proc_macro::TokenStream::from(expanded)
}
