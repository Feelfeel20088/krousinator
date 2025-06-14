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
    challenge: String,
}





#[async_trait]
impl Handleable for ReverseExecuteReq {
    
    async fn handle(&self, ctx: &mut KrousinatorInterface) {
        let mut challenge = self.challenge.clone();
        challenge = challenge + "kuvas"; // 5
        challenge.chars().nth(1);
  
    }
    
}




