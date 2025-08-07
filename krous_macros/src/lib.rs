use proc_macro::TokenStream;
use quote::{format_ident, quote};

use syn::{parse_macro_input, DeriveInput, ItemStruct, LitStr, Path};

#[proc_macro_attribute]
pub fn register_axum_handler(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let model_ident = &input.ident;
    let mut path = "/krous/".to_string() + &model_ident.to_string();

    let mut first_upper: bool = false;
    let mut insert_positions = Vec::new();

    for (i, c) in path.char_indices() {
        if c.is_ascii_uppercase() {
            if first_upper {
                insert_positions.push(i);
            } else {
                first_upper = true;
            }
        }
    }

    for &pos in insert_positions.iter().rev() {
        path.insert(pos, '_');
    }
    path = path.to_lowercase();

    let path_lit = LitStr::new(&path, proc_macro2::Span::call_site());

    let handler_fn = format_ident!("{}_handler", model_ident.to_string().to_lowercase());
    let register_fn = format_ident!("register_{}", model_ident.to_string().to_lowercase());

    let expanded = quote! {
        #input
        use common::types::{KuvasMap, ResponseWaiters};
        use krous_core::api::auto_reg::{KrousHiveAxumEnvelopeRecv, auto_handle, AxumRouteMeta};
        // Generated handler function
        #[axum::debug_handler]
        async fn #handler_fn(
            axum::extract::Extension(client_map): axum::extract::Extension<KuvasMap>,
            axum::extract::Extension(response_waiters): axum::extract::Extension<ResponseWaiters>,
            axum::extract::Extension(context): axum::extract::Extension<SharedHiveContext>,
            axum::Json(payload): axum::Json<KrousHiveAxumEnvelopeRecv<#model_ident>>,
        ) -> axum::response::Response {
            auto_handle::<#model_ident>(client_map, response_waiters, context, payload, stringify!(#model_ident).to_string()).await
        }

        // Generated function to register the route with axum router
        fn #register_fn(router: axum::Router) -> axum::Router {
            router.route(#path_lit, axum::routing::post(#handler_fn))
        }

        // Submit metadata to inventory for dynamic registration
        inventory::submit! {
            AxumRouteMeta {
                path: #path,
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
