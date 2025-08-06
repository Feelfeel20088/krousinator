use crate::registry::Context;
use crate::types::SharedHiveContext;
use async_trait::async_trait;

#[async_trait]
pub trait Handleable: Send + Sync {
    async fn handle(&self, ctx: &mut Context);
}
