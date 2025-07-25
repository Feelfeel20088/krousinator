use async_trait::async_trait;
use common::registry::HiveHandleable;
use common::types::SharedHiveContext;
use krous_macros::{register_axum_handler, register_hive_handler};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[register_hive_handler]
pub struct IdentityReqRecv {
    _t: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[register_axum_handler]
pub struct IdentityResponseSend {
    _t: String,
}

#[async_trait]
impl HiveHandleable for IdentityReqRecv {
    async fn handle(&self, ctx: SharedHiveContext) {}
}
