use async_trait::async_trait;

use crate::registry::Context;

pub trait Producer {
    fn produce(krousinator_instance_data: &Context) -> Self
    where
        Self: Sized;
}
