use proc_macro::TokenStream;
use quote::{format_ident, quote};

use syn::{parse_macro_input, DeriveInput, ItemStruct};

#[proc_macro_attribute]
pub fn register_axum_handler(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the macro attribute input as AttributeArgs (list of nested meta)
    let path_lit = stringify!(attr);
    // Parse the item the attribute is applied to (expecting a struct or enum)
    let input = parse_macro_input!(item as DeriveInput);

    let model_ident = &input.ident;

    let handler_fn = format_ident!("{}_handler", model_ident.to_string().to_lowercase());
    let register_fn = format_ident!("register_{}", model_ident.to_string().to_lowercase());

    let expanded = quote! {
        #input
        use common::types::{KuvasMap, ResponseWaiters};
        use common::axum_register::temp::{KrousHiveEnvelope, build_handler, AxumRouteMeta};
        // Generated handler function
        async fn #handler_fn(
            axum::extract::Extension(client_map): axum::extract::Extension<KuvasMap>,
            axum::extract::Extension(response_waiters): axum::extract::Extension<ResponseWaiters>,
            axum::extract::Extension(context): axum::extract::Extension<SharedHiveContext>,
            axum::Json(payload): axum::Json<KrousHiveEnvelope<#model_ident>>,
        ) -> axum::response::Response {
            build_handler::<#model_ident>(client_map, response_waiters, context, payload).await
        }

        // Generated function to register the route with axum router
        fn #register_fn(router: axum::Router) -> axum::Router {
            router.route(#path_lit, axum::routing::post(#handler_fn))
        }

        // Submit metadata to inventory for dynamic registration
        inventory::submit! {
            AxumRouteMeta {
                path: #path_lit,
                register_fn: #register_fn,
            }
        }
    };

    TokenStream::from(expanded)
}

// handler for auto serd

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

#[proc_macro_attribute]
pub fn register_hive_handler(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let struct_name = &input.ident;

    let expanded = quote! {
        #input

        inventory::submit! {
            common::registry::HiveHandlerMeta {
                name: stringify!(#struct_name),
                constructor: |json| {
                    match serde_json::from_str::<#struct_name>(json) {
                        Ok(model) => Ok(Box::new(model) as Box<dyn common::registry::HiveHandleable + Send + Sync>),
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
