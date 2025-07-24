use crate::{
    registry::{HiveContext, HiveHandleable},
    types::{KuvasMap, ResponseWaiters, SharedHiveContext},
};
use axum::{http::StatusCode, response::IntoResponse, Router};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct KrousHiveEnvelope<T> {
    pub krous_id: String,
    #[serde(flatten)]
    pub model: T,
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
pub async fn build_handler<T>(
    client_map: KuvasMap,
    response_waiters: ResponseWaiters,
    context: SharedHiveContext,
    payload: KrousHiveEnvelope<T>,
) -> impl IntoResponse
where
    T: HiveHandleable + Serialize + DeserializeOwned + Send + Sync + 'static,
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

    let recv_model = match HiveContext::send_request_to_krousinator::<T>(
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
