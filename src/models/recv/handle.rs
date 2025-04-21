#[typetag::serde(tag = "type")]
pub trait Handleable {
    fn handle(&self);
}
