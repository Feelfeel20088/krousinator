use serde::Deserialize;
use async_trait::async_trait;
use uuid::Uuid;
use krous_macros::register_handler;

use common::{
    registry::{
        Handleable,
        Context,
    },
};





#[derive(Deserialize, Debug)]
#[register_handler]
pub struct AuthReqRecv {
    _t: String,
    challenge: String,
    manual_request_id: Option<Uuid>
}





#[async_trait]
impl Handleable for AuthReqRecv {
    
    async fn handle(&self, ctx: &mut Context) {
        let mut challenge = self.challenge.clone();
        challenge = challenge + "kuvas"; // 5
        challenge.chars().nth(1);
  
    }
    
}




