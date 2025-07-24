use async_trait::async_trait;
use common::registry::{HiveContext, HiveHandleable};
use common::types::SharedHiveContext;
use krous_macros::{register_axum_handler, register_hive_handler};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
#[register_hive_handler]
#[register_axum_handler("/testpath")]
pub struct ReverseExecuteRecv {
    _t: String,
    manual_request_id: Option<Uuid>,
    pub successful: bool,
    pub uuid: Uuid,
    pub response: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct ReverseExecuteSend {
    _t: String,
    payload: String,        // full command
    payload_response: bool, // to send back the shells output or not
    manual_request_id: Option<Uuid>,
}

#[async_trait]
impl HiveHandleable for ReverseExecuteRecv {
    async fn handle(&self, ctx: SharedHiveContext) {
        // store in database somewhere
    }
}
