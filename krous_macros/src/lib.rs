
use common::types::{ResponseWaiters, KuvasMap};
use HiveContext::send_request_to_krousinator;


use proc_macro::TokenStream;
use syn::{
    parse_macro_input,      
    DeriveInput,                                    
    ItemStruct,              
};
use quote::{quote, format_ident};

use crate::KrousHiveEnvelope;
use crate::build_handler;

#[derive(Deserialize)]
pub struct KrousHiveEnvelope<T> 
{
    krous_id: String,
    #[serde(flatten)]
    model: T,
}

pub struct AxumRouteHander {
    pub path: &'static str,
    pub register_fn: fn(Router) -> Router,
}

inventory::collect!(AxumRouteHander);

// currently there is no check to see the model being passed in is a valid model.
// front end softwhere will recv something back from the krousinator like { error: model not valid }
// although this should never happen unless someone messes up the frontend code or someone is trying to use
// the api
async fn build_handler<T>(
    client_map: KuvasMap,
    response_waiters: ResponseWaiters,
    context: SharedHiveContext,
    payload: KrousHiveEnvelope<T>,
) -> impl IntoResponse
where T: HiveHandleable + Serialize + DeserializeOwned + Send + Sync + 'static
{
    let krous_uuid = match Uuid::parse_str(&payload.krous_id) {
        Ok(uuid) => uuid,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                format!("Krousinator id {} is not a valid UUID: ", &payload.krous_id),
            )
                .into_response();
        }
    };

    let inner_json: String = match serde_json::to_string(&payload.model) {
        Ok(inner) => inner,
        Err(_) => { 
            return (
                StatusCode::BAD_REQUEST,
                "Model sent is not valid json".to_string(),
            )
                .into_response();
        }
    };

    let recv_model = match send_request_to_krousinator(
        krous_uuid,
        client_map,
        response_waiters,
        inner_json,
    )
    .await
    {
        Ok(model) => model,
        Err(err) => return err.into_response(),
    };
    // this will go down the stack sending and recving more 
    // model until the orginal recv model returns the resulting struct 
    // NOTE TO SELF. there is currently know way for models to add to themselfs like collecting 
    // more info as it sends and recvs more models. it may be approite to return a diffrent type
    // that each model defines as its resulting thingy 
    recv_model.handle(context).await;

    (StatusCode::OK, "Success".to_string()).into_response()
}





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
            common::registry::AxumRouteMeta {
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
