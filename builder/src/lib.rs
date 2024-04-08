use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

// pub struct DeriveInput {
//    pub attrs: Vec<Attribute>,
//    pub vis: Visibility,
//    pub ident: Ident,
//    pub generics: Generics,
//    pub data: Data,
//}

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let original_name = input.ident;
    let builder_name = quote::format_ident!("{}Builder", original_name);

    let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = input.data
    else {
        panic!("Argument must be struct");
    };

    let builder_init = named.iter().map(|f| {
        let identity = &f.ident;
        quote! {
            #identity: None,
        }
    });

    let builder_fields = named.iter().map(|f| {
        let identity = &f.ident;
        let typing = &f.ty;
        quote! {
            #identity: Option<#typing>,
        }
    });

    let builder_fn = named.iter().map(|f| {
        let identity = &f.ident;
        let typing = &f.ty;
        quote! {
            fn #identity(&mut self, #identity: #typing) -> &mut Self {
                self.#identity = Some(#identity);
                self
            }
        }
    });

    let builder_build = named.iter().map(|f| {
        let ident = &f.ident;
        quote! {
            #ident: self.#ident.take().ok_or("#ident not set")?,
        }
    });

    let expanded = quote! {
        impl #original_name {
            fn builder() -> #builder_name {
                #builder_name {
                    #(#builder_init)*
                }
            }
        }

        pub struct #builder_name {
            #(#builder_fields)*
        }

       impl #builder_name {
           #(#builder_fn)*
           pub fn build(&mut self) -> Result<#original_name, Box<dyn std::error::Error>> {
               Ok(#original_name {
                   #(#builder_build)*
               })
           }
       }

    };
    expanded.into()
}
