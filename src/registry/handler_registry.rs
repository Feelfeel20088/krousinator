use std::collections::HashMap;
use crate::registry::handle::Handleable;

type DynHandlerConstructor = fn(&str) -> Box<dyn Handleable>;

pub struct HandlerRegistry {
    map: HashMap<String, DynHandlerConstructor>,
}

impl HandlerRegistry {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: &str, constructer: fn(&str) -> Box<dyn Handleable>)
    {
        self.map.insert(name.to_string(), constructer);
        println!("{}", name.to_string());
    }

    pub fn get(&self, name: &str, json: &str) -> Option<Box<dyn Handleable>> {
        self.map.get(name).map(|ctor| ctor(json))
    }
}
