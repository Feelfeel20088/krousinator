
pub trait Producer {
    fn produce() -> Box<dyn Producer>
    where
        Self: Sized;
    
}

pub trait Serd {
    fn serd(&self) -> Result<String, String> 
    where Self: serde::Serialize,
    {

        match serde_json::to_string(self) {
            Ok(json) => {
                return Ok(json);
            },
            Err(e) => return Err(format!("Error serializing: {}", e).to_string()),
        }
    
    }
    
}