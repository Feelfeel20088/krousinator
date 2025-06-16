use crate::registry::{handle::Handleable, HiveHandleable};

pub type DynHandlerConstructor = fn(&str) -> Result<Box<dyn Handleable + Send + Sync>, serde_json::Error>;
pub type DynHiveHandlerConstructor = fn(&str) -> Result<Box<dyn HiveHandleable + Send + Sync>, serde_json::Error>;

// name of struct is type
pub struct HandlerMeta {
    pub name: &'static str,
    pub constructor: DynHandlerConstructor,
}

inventory::collect!(HandlerMeta);

pub struct HiveHandlerMeta {
    pub name: &'static str,
    pub constructor: DynHiveHandlerConstructor,
}

inventory::collect!(HiveHandlerMeta);
