use crate::api::model::traits::handlers::{Handleable, HiveHandleable};
use std::collections::HashMap;

pub type DynHiveHandlerConstructor =
    fn(&str) -> Result<Box<dyn HiveHandleable + Send + Sync + 'static>, serde_json::Error>;

pub struct HiveHandlerRegistry {
    factories: HashMap<String, DynHiveHandlerConstructor>,
}

impl HiveHandlerRegistry {
    pub fn new() -> Self {
        Self {
            factories: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: &str, constructer: DynHiveHandlerConstructor) {
        self.factories.insert(name.to_string(), constructer);
    }

    pub fn get(
        &self,
        name: &str,
        json: &str,
    ) -> Option<Result<Box<dyn HiveHandleable + Send + Sync + 'static>, serde_json::Error>> {
        self.factories.get(name).map(|ctor| ctor(json))
    }

    pub fn check(&self, name: &str) -> bool {
        self.factories.contains_key(name)
    }
}

pub type DynHandlerConstructor =
    fn(&str) -> Result<Box<dyn Handleable + Send + Sync + 'static>, serde_json::Error>;

pub struct HandlerRegistry {
    factories: HashMap<String, DynHandlerConstructor>,
}

impl HandlerRegistry {
    pub fn new() -> Self {
        Self {
            factories: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: &str, constructer: DynHandlerConstructor) {
        self.factories.insert(name.to_string(), constructer);
    }

    pub fn get(
        &self,
        name: &str,
        json: &str,
    ) -> Option<Result<Box<dyn Handleable + Send + Sync + 'static>, serde_json::Error>> {
        self.factories.get(name).map(|ctor| ctor(json))
    }
}
