use krous_macros::register_handler;
use serde::Deserialize;
use async_trait::async_trait;
use uuid::Uuid;

use common::{
    registry::{
        Handleable,
        Context,
    },
};





#[derive(Deserialize, Debug)]
#[register_handler]
pub struct IdentityResponseRecv {
    _t: String,
    id: Uuid,
}





#[async_trait]
impl Handleable for IdentityResponseRecv {
    
    async fn handle(&self, ctx: &mut Context) {
        ctx.set_uuid(self.id);
    }
    
}




