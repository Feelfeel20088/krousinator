use crate::registry::Context;
use crate::types::SharedHiveContext;
use async_trait::async_trait;

#[async_trait]
pub trait Handleable: Send + Sync {
    async fn handle(&self, ctx: &mut Context);
}

#[async_trait]
pub trait HiveHandleable: Send + Sync {
    async fn handle(&self, ctx: SharedHiveContext);

    // figuer out what i need to pass to this guy. probely add my utility functions
    // into hivecontext so it can send other
}
