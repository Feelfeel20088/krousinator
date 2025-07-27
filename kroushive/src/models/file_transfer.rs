use async_trait::async_trait;
use common::registry::HiveHandleable;
use common::types::SharedHiveContext;
use krous_macros::{register_axum_handler, register_hive_handler};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[register_hive_handler]
pub struct FileTransferRecv {
    pub successful: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[register_axum_handler]
pub struct FileTransferSend {
    binary: Vec<u8>, // full binary, image or any file u want to send over.
}

#[async_trait]
impl HiveHandleable for FileTransferRecv {
    async fn handle(&self, ctx: SharedHiveContext) {
        // store in database somewhere
    }
}
