
pub trait Producer {
    fn produce() -> Self
    where
        Self: Sized;
    
}
