use async_trait::async_trait;
use common::registry::HiveHandleable;
use common::types::SharedHiveContext;
use krous_macros::register_hive_handler;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
#[register_hive_handler]
pub struct ReverseExecuteRecv {
    _t: String,
    pub successful: bool,
    pub uuid: Uuid,
    pub response: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct ReverseExecuteSend {
    _t: String,
    payload: String,        // full command
    payload_response: bool, // to send back the shells output or not
}

#[async_trait]
impl HiveHandleable for ReverseExecuteRecv {
    async fn handle(&self, ctx: SharedHiveContext) {
        // store in database somewhere
    }
}
