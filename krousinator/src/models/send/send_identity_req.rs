use serde::{Serialize, Deserialize};
use common::registry::producer::Producer;

use common::registry::krousinator_interface::KrousinatorInterface;

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityReqSend {
    _t:  &'static str,
}

impl Producer for IdentityReqSend {
    fn produce(_krousinator_instance_data: &KrousinatorInterface) -> Self {
        Self { _t: "IdentityReqSend"}
        
    }
}

