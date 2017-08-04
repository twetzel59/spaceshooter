#[derive(Clone, Copy)]
pub enum Status {
    Dead,
    Alive,
}

pub trait Attackable {
    fn die(&mut self);
    
    fn check(&self) -> Status;
}
