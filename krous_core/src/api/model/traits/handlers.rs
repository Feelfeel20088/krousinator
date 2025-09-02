use crate::context::context::Context;
use crate::context::hive_context::HiveContext;

use async_trait::async_trait;

#[async_trait]
pub trait HiveHandleable: Send + Sync {
    async fn handle(&self, ctx: HiveContext);

    // figuer out what i need to pass to this guy. probely add my utility functions
    // into hivecontext so it can send other
}

#[async_trait]
pub trait Handleable: Send + Sync {
    async fn handle(&self, ctx: &mut Context);
}
