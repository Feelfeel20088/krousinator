use crate::registry::krousinator_interface::KrousinatorInterface;

pub trait Handleable {
    fn handle(&self, ctx: &mut KrousinatorInterface);
}
