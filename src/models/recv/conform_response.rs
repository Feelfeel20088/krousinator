use crate::registry::handle::Handleable;
use crate::registry::krousinator_interface::KrousinatorInterface;
use futures_util::stream::Any;
use serde::Deserialize;
use crate::register_handler;

#[derive(Deserialize, Debug)]
pub struct ConformResponse {
    t: String,
    msg: String
}


impl Handleable for ConformResponse {
    fn handle(&self, ctx: &mut KrousinatorInterface) {
        return; // do nothing
    }
}

register_handler!(ConformResponse);