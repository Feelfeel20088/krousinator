use async_trait::async_trait;

use crate::context::{context::Context, hive_context::HiveContext};

#[async_trait]
pub trait HiveProducer: Sized {
    fn produce(krousinator_instance_data: &HiveContext) -> Self
    where
        Self: Sized;
    // your methods
}

pub trait Producer {
    fn produce(krousinator_instance_data: &Context) -> Self
    where
        Self: Sized;
}
