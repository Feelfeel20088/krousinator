#[async_trait]
pub trait HiveHandleable: Send + Sync {
    async fn handle(&self, ctx: SharedHiveContext);

    // figuer out what i need to pass to this guy. probely add my utility functions
    // into hivecontext so it can send other
}
