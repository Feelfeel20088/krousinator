use async_trait::async_trait;
use krous_macros::register_handler;
use serde::Deserialize;
use uuid::Uuid;

use common::registry::{Context, Handleable};

#[derive(Deserialize, Debug)]
#[register_handler]
pub struct AuthReqRecv {
    _t: String,
    challenge: String,
}

#[async_trait]
impl Handleable for AuthReqRecv {
    async fn handle(&self, ctx: &mut Context) {
        let mut challenge = self.challenge.clone();
        challenge = challenge + "kuvas"; // 5
        challenge.chars().nth(1);
    }
}
