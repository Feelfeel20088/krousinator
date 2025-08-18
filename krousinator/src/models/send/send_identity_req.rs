use serde::{Serialize, Deserialize};
use common::registry::{Producer, Context};

#[derive(Debug, Serialize, Deserialize)]

pub struct IdentityReqSend {
    _t:  &'static str,
}

impl Producer for IdentityReqSend {
    fn produce(_krousinator_instance_data: &Context) -> Self {
        Self { _t: "IdentityReqSend"}
        
    }
}

