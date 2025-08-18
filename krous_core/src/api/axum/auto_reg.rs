use crate::{
    context::hive_context::HiveContext,
    types::{KuvasMap, ResponseWaiters, SharedHiveContext},
};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Router,
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
enum KrousId {
    Id(String),
    Broadcast,
}

#[derive(Deserialize)]
pub struct KrousHiveAxumEnvelopeRecv<T> {
    pub krous_id: KrousId,
    pub model: T,
}

pub struct AxumRouteMeta {
    pub path: &'static str,
    pub register_fn: fn(Router) -> Router,
}

inventory::collect!(AxumRouteMeta);

// currently there is no check to see the model being passed in is a valid model.
// front end softwhere will recv something back from the krousinator like { error: model not valid }
// although this should never happen unless someone messes up the frontend code or someone is trying to use
// the api
pub async fn auto_handle<T>(
    client_map: KuvasMap,
    response_waiters: ResponseWaiters,
    context: SharedHiveContext,
    payload: KrousHiveAxumEnvelopeRecv<T>,
    type_name: String,
) -> Response
where
    T: Serialize + Send + Sync + 'static,
{
    match payload.krous_id {
        KrousId::Id(id) => {
            let krous_uuid = match Uuid::parse_str(&id) {
                Ok(uuid) => uuid,
                Err(_) => {
                    return (
                        StatusCode::BAD_REQUEST,
                        format!("Krousinator id {} is not a valid UUID: ", id),
                    )
                        .into_response();
                }
            };

            let recv_model = match HiveContext::send_request_to_krousinator::<T>(
                krous_uuid,
                client_map,
                response_waiters,
                payload.model,
                type_name,
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
            recv_model.model.handle(context).await;

            return (StatusCode::OK, "Success".to_string()).into_response();
        }
        KrousId::Broadcast => {
            todo!()
            // let recv_model = match HiveContext::send_request_to_krousinator::<T, T2>(
            //     krous_uuid,
            //     client_map,
            //     response_waiters,
            //     payload.model,
            // )
            // .await
            // {
            //     Ok(model) => model,
            //     Err(err) => return err.into_response(),
            // };
            // // this will go down the stack sending and recving more
            // // model until the orginal recv model returns the resulting struct
            // // NOTE TO SELF. there is currently know way for models to add to themselfs like collecting
            // // more info as it sends and recvs more models. it may be approite to return a diffrent type
            // // that each model defines as its resulting thingy
            // recv_model.handle(context).await;

            // return (StatusCode::OK, "Success".to_string()).into_response();
        }
    }
}
