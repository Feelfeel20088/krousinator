use uuid::Uuid;
use serde::{Serialize, Deserialize};
use krous_macros::register_hive_handler;
use async_trait::async_trait;
use common::registry::{HiveContext, HiveHandleable};

#[derive(Debug, Deserialize)]
#[register_hive_handler]
pub struct ReverseExecuteRecv {
    _t: String,
    manual_request_id: Option<Uuid>,
    pub successful: bool,
    pub uuid: Uuid,
    pub response: Option<String> 
 
}

#[derive(Serialize, Debug)]
pub struct ReverseExecuteSend {
    _t: String,
    payload: String, // full command
    payload_response: bool, // to send back the shells output or not
    manual_request_id: Option<Uuid>
}


#[async_trait]
impl HiveHandleable for ReverseExecuteRecv {
    async fn handle(&self, ctx: &HiveContext) {
        // store in database somewhere
    }


}
