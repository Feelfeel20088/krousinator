use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn register_handler(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let struct_name = &input.ident;


    let expanded = quote! {
        #input
    
        inventory::submit! {
            common::registry::HandlerMeta {
                name: stringify!(#struct_name),
                constructor: |json| {
                    match serde_json::from_str::<#struct_name>(json) {
                        Ok(model) => Ok(Box::new(model) as Box<dyn common::registry::Handleable + Send + Sync>),
                        Err(e) => {
                            #[cfg(debug_assertions)]
                            {
                                use colored::Colorize;
                                eprintln!(
                                    "{} Failed to deserialize JSON into model {}.\n\
                                     {}\n\
                                     └─ Check the C&C server for mismatches in the sent data, or inspect the corresponding Krousinator handler implementation.",
                                    "[ERROR]".red().bold(),
                                    stringify!(#struct_name).yellow(),
                                    "The `_t` field matches, but the structure does not.".blue()
                                );
                            }
                            Err(e)                            
                        },
                    }
                }
            }
        }
    };
    

    TokenStream::from(expanded)
}

