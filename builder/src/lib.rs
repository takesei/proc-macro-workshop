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
    let name = input.ident;

    let expanded = quote! {
        pub struct CommandBuilder {
            executable: Option<String>,
            args: Option<Vec<String>>,
            env: Option<Vec<String>>,
            current_dir: Option<String>,
        }

       impl CommandBuilder {
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
       }


        impl #name {
            fn builder() -> CommandBuilder {
                CommandBuilder {
                    executable: None,
                    args: None,
                    env: None,
                    current_dir: None
                }
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}
