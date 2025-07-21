use async_trait::async_trait;
use common::registry::{HiveContext, HiveHandleable};
use krous_macros::{register_hive_handler, register_axum_handler};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use common::types::SharedHiveContext;

#[derive(Debug, Deserialize)]
#[register_hive_handler]
#[register_axum_handler("/testpath")]
pub struct IdentityReqRecv {
    _t: String,
}

#[derive(Debug, Serialize)]
pub struct IdentityResponseSend {
    _t: String,
    manual_request_id: Uuid,
}

#[async_trait]
impl HiveHandleable for IdentityReqRecv {
    async fn handle(&self, ctx: SharedHiveContext) {}

}
