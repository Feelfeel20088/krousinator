#[typetag::serde(tag = "type")]
pub trait Producer {
    fn produce() -> Box<dyn Producer>
    where
        Self: Sized;
    fn send(&self);
}