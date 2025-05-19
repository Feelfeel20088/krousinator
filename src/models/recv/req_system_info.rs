use crate::models::send::send_model_traits::Producer;
use crate::registry::handle::Handleable;
use crate::registry::krousinator_interface::KrousinatorInterface;
use serde::Deserialize;
use crate::register_handler;
use crate::models::send::send_system_info::SystemInfoSend;

#[derive(Deserialize, Debug)]
pub struct SystemInfoReq {
    t: String,
}


impl Handleable for SystemInfoReq {
    fn handle(&self, ctx: &mut KrousinatorInterface) {
        let send_system_info_object = SystemInfoSend::produce();
        ctx.send(send_system_info_object);
        println!("sent paylaod!");


    }
}

register_handler!(SystemInfoReq);