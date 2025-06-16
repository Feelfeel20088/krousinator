use crate::registry::{Context, HiveContext};
use async_trait::async_trait;


#[async_trait]
pub trait Handleable: Send + Sync {
    async fn handle(&self, ctx: &mut Context);
}

#[async_trait]
pub trait HiveHandleable: Send + Sync {
    async fn handle(&self, ctx: &HiveContext);
}

 