use crate::registry::handle::Handleable;
use crate::registry::krousinator_interface::KrousinatorInterface;
use serde::Deserialize;
use crate::register_handler;

#[derive(Deserialize, Debug)]
pub struct HandleTest {
    t: String,
    msg: String
}


impl Handleable for HandleTest {
    fn handle(&self, ctx: &mut KrousinatorInterface) {
        println!("handle test was called")
    }
}

register_handler!(HandleTest);