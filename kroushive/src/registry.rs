pub type DynHiveHandlerConstructor =
    fn(&str) -> Result<Box<dyn HiveHandleable + Send + Sync + 'static>, serde_json::Error>;

pub struct HiveHandlerRegistry {
    map: HashMap<String, DynHiveHandlerConstructor>,
}

impl HiveHandlerRegistry {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: &str, constructer: DynHiveHandlerConstructor) {
        self.map.insert(name.to_string(), constructer);
    }

    pub fn get(
        &self,
        name: &str,
        json: &str,
    ) -> Option<Result<Box<dyn HiveHandleable + Send + Sync + 'static>, serde_json::Error>> {
        self.map.get(name).map(|ctor| ctor(json))
    }

    pub fn check(&self, name: &str) -> bool {
        self.map.contains_key(name)
    }
}
