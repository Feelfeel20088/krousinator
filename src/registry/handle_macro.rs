#[macro_export]
macro_rules! register_handler {
    ($struct:ty) => {
        inventory::submit! {
            $crate::registry::entry::HandlerMeta {
                name: stringify!($struct),
                constructor: |json| {
                    let model: $struct = serde_json::from_str(json).expect("Invalid JSON");
                    Box::new(model)
                }
            }
        }
    };
}
