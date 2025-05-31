use crate::registry::handle::Handleable;

pub type DynHandlerConstructor = fn(&str) -> Box<dyn Handleable>;

// name of struct is type
pub struct HandlerMeta {
    pub name: &'static str,
    pub constructor: DynHandlerConstructor,
}

inventory::collect!(HandlerMeta);
