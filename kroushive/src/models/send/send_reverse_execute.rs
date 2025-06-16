use common::registry::{HiveContext, HiveProducer};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReverseExecuteSend {
    _t: String,
    payload: String, // full command
    payload_response: bool, // to send back the shells output or not
    manual_request_id: Option<Uuid>
}

