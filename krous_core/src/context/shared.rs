use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    api::model::traits::handlers::HiveHandleable, registry::registry::HiveHandlerRegistry,
};

// add more things as needed it should be meta data that needs to be included in
// every req to both krousinator and kroushive
#[derive(Serialize, Deserialize)]
pub struct KrousEnvelopeSend<T> {
    pub manual_request_id: Option<Uuid>,
    pub id: Uuid,
    pub _t: String,
    pub model: T,
}
#[derive(serde::Deserialize)]
struct KrousEnvelopeHelper {
    pub manual_request_id: Option<Uuid>,
    pub id: Uuid,
    pub _t: String,
    pub model: String,
}

pub struct KrousEnvelopeRecv {
    pub manual_request_id: Option<Uuid>,
    pub id: Uuid,
    pub _t: String,

    pub model: Box<dyn HiveHandleable + Send + Sync>,
}

impl KrousEnvelopeRecv {
    pub fn deserialize(deserializer: String, reg: &HiveHandlerRegistry) -> Result<Self, Error> {
        let helper = match serde_json::from_str::<KrousEnvelopeHelper>(&deserializer) {
            Ok(helper) => helper,
            Err(e) => return Err(e),
        };

        // You can switch based on `_t` or just use a default model
        let model: Box<dyn HiveHandleable + Send + Sync> = match reg.get(&helper._t, &helper.model)
        {
            Some(Ok(model)) => model,

            Some(Err(e)) => {
                println!(
                "Error: Model failed to deserialize.\n\
                This is not normal behavior and most likely indicates the client is sending malformed payloads.\n\
                This could mean the client code is broken, or someone is attempting to simulate a client."
                );
                return e;
            }

            None => {
                println!(
                    "Error: model type '{}' was not found in the registry.",
                    &helper._t
                );
                return Error;
            }
        };

        Ok(KrousEnvelopeRecv {
            manual_request_id: helper.manual_request_id,
            id: helper.id,
            _t: helper._t,
            model,
        })
    }
}

impl<T> KrousEnvelopeSend<T>
where
    T: Serialize + Send + Sync + 'static,
{
    pub fn new(manual_request_id: Option<Uuid>, id: Uuid, _t: String, model: T) -> Self {
        Self {
            manual_request_id,
            id,
            _t,
            model,
        }
    }

    pub fn serd(self) -> Result<String, (StatusCode, std::string::String)> {
        match serde_json::to_string(&self) {
            Ok(inner) => Ok(inner),
            Err(_) => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    "Model sent is not valid json".to_string(),
                ))
            }
        }
    }
}
