#[macro_export]
macro_rules! register_handler {
    ($ty:ty, $name:expr) => {
        inventory::submit! {
            $crate::registry::entry::HandlerMeta {
                name: $name,
                constructor: |json| {
                    let model: $ty = serde_json::from_str(json).expect("Invalid JSON");
                    Box::new(model)
                }
            }
        }
    };
}
