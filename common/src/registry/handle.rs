use crate::registry::krousinator_interface::KrousinatorInterface;
use async_trait::async_trait;


#[async_trait]
pub trait Handleable: Send + Sync{
    async fn handle(&self, ctx: &mut KrousinatorInterface);
}
