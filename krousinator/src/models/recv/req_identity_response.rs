use krous_macros::register_handler;
use serde::{Serialize, Deserialize};
use async_trait::async_trait;
use uuid::Uuid;

use common::{
    registry::{
        handle::Handleable,
        krousinator_interface::KrousinatorInterface,
    },
};





#[derive(Deserialize, Debug)]
#[register_handler]
pub struct ReverseExecuteReq {
    _t: String,
    uuid: Uuid,
}





#[async_trait]
impl Handleable for ReverseExecuteReq {
    
    async fn handle(&self, ctx: &mut KrousinatorInterface) {
        ctx.set_uuid(self.uuid);
    }
    
}




