pub mod registry;

pub fn init_kroushive_registry() -> registry::HiveHandlerRegistry {
    let mut reg = registry::HiveHandlerRegistry::new();
    for handler in inventory::iter::<crate::api::model::meta::HiveHandlerMeta> {
        reg.register(handler.name, handler.constructor);
    }
    return reg;
}
