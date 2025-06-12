
use crate::registry::krousinator_interface::KrousinatorInterface;

pub trait Producer {
    fn produce(krousinator_instance_data: &KrousinatorInterface) -> Self
    where
        Self: Sized;
    
}
