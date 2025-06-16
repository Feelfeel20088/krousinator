use std::collections::HashMap;
use crate::registry::Handleable;

pub type DynHandlerConstructor = fn(&str) -> Result<Box<dyn Handleable + Send + Sync>, serde_json::Error>;

pub struct HandlerRegistry {
    map: HashMap<String, DynHandlerConstructor>,
}

impl HandlerRegistry {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: &str, constructer: DynHandlerConstructor)
    {
        self.map.insert(name.to_string(), constructer);
        println!("{}", name.to_string());
    }

    pub fn get(&self, name: &str, json: &str) -> Option<Result<Box<dyn Handleable + Send + Sync>, serde_json::Error>> {
        self.map.get(name).map(|ctor| ctor(json))
    }
}
