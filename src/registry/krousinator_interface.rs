use crate::models::send::send_model_traits::Producer;
use serde_json;
pub struct KrousinatorInterface;

impl KrousinatorInterface {
    pub fn new() -> Self {
        KrousinatorInterface {}
    }

    pub fn send(send_object: Box<dyn Producer>) {
        print!("sending object")
    }
}