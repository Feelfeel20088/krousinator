use crate::models::recv::handle::Handleable;
use serde::de::DeserializeOwned;

pub type DynHandlerConstructor = fn(&str) -> Box<dyn Handleable>;

pub struct HandlerMeta {
    pub name: &'static str,
    pub constructor: DynHandlerConstructor,
}

inventory::collect!(HandlerMeta);
