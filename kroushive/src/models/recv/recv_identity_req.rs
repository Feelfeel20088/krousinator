use common::registry::{HiveContext, HiveHandleable, HiveProducer};
use serde::{Serialize, Deserialize};
use krous_macros::register_hive_handler;
use async_trait::async_trait;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[register_hive_handler]

pub struct IdentityReqRecv {
    _t:  String
}

pub struct IdentityResponseSend {
    _t: String,
    id: Uuid,
}

#[async_trait]
impl HiveHandleable for IdentityReqRecv {
    async fn handle(&self, ctx: &HiveContext) {
        
    }
}

