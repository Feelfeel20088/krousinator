use crate::registry::{Handleable, HiveHandleable};
use std::collections::HashMap;

pub type DynHandlerConstructor =
    fn(&str) -> Result<Box<dyn Handleable + Send + Sync + 'static>, serde_json::Error>;

pub struct HandlerRegistry {
    map: HashMap<String, DynHandlerConstructor>,
}

impl HandlerRegistry {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: &str, constructer: DynHandlerConstructor) {
        self.map.insert(name.to_string(), constructer);
    }

    pub fn get(
        &self,
        name: &str,
        json: &str,
    ) -> Option<Result<Box<dyn Handleable + Send + Sync + 'static>, serde_json::Error>> {
        self.map.get(name).map(|ctor| ctor(json))
    }
}
