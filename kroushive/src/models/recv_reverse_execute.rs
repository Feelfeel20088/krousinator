use async_trait::async_trait;
use krous_core::{
    api::model::traits::handlers::HiveHandleable, context::hive_context::HiveContext,
};
use krous_macros::{register_axum_handler, register_hive_handler};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[register_hive_handler]
pub struct ReverseExecuteRecv {
    pub successful: bool,
    pub response: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[register_axum_handler]
pub struct ReverseExecuteSend {
    payload: String,        // full command
    payload_response: bool, // to send back the shells output or not
}

#[async_trait]
impl HiveHandleable for ReverseExecuteRecv {
    async fn handle(&self, ctx: HiveContext) {
        // store in database somewhere
    }
}
