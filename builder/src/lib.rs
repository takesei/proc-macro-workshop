use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, token::Struct, DataStruct, DeriveInput, FieldsNamed};

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

    let builder_fields = named.iter().map(|f| {
        let ident = &f.ident;
        quote! {
            #ident: None
        }
    });

    let expanded = quote! {
        pub struct #builder_name {
            executable: Option<String>,
            args: Option<Vec<String>>,
            env: Option<Vec<String>>,
            current_dir: Option<String>,
        }

       impl #builder_name {
           fn executable(&mut self, executable: String) -> &mut Self {
               self.executable = Some(executable);
               self
           }
           fn args(&mut self, args: Vec<String>) -> &mut Self {
               self.args = Some(args);
               self
           }
           fn env(&mut self, env: Vec<String>) -> &mut Self {
               self.env = Some(env);
               self
           }
           fn current_dir(&mut self, current_dir: String) -> &mut Self {
               self.current_dir = Some(current_dir);
               self
           }
           pub fn build(&mut self) -> Result<#original_name, Box<dyn std::error::Error>> {
               let executable = self.executable.take().ok_or("executable")?;
               let args =  self.args.take().ok_or("executable")?;
               let env = self.env.take().ok_or("env")?;
               let current_dir = self.current_dir.take().ok_or("current_dir")?;

               Ok(#original_name {
                   executable: executable,
                   args: args,
                   env: env,
                   current_dir: current_dir,
               })

           }
       }


        impl #original_name {
            fn builder() -> #builder_name {
                #builder_name {
                    #(#builder_fields,)*
                }
            }
        }
    };
    expanded.into()
}
