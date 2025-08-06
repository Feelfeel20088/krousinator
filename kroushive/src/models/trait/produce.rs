#[async_trait]
pub trait HiveProducer: Sized {
    fn produce(krousinator_instance_data: &HiveContext) -> Self
    where
        Self: Sized;
    // your methods
}
