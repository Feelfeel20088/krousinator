use crate::registry::handle::Handleable;

pub type DynHandlerConstructor = fn(&str) -> Result<Box<dyn Handleable + Send + Sync>, serde_json::Error>;

// name of struct is type
pub struct HandlerMeta {
    pub name: &'static str,
    pub constructor: DynHandlerConstructor,
}

inventory::collect!(HandlerMeta);
