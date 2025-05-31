use std::any::Any;

use serde::{Serialize, Deserialize};
use super::producer::Producer;
use arboard::Clipboard;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfirmResponseSend {
    pub successful: bool,
    pub error: Option<String> // pipe for errors can be any type of error from any model
}

impl Producer for ConfirmResponseSend {
    fn produce() -> Self {
        Self {successful: false, error: None}
    }
}

