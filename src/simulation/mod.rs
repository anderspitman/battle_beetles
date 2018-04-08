pub mod speed_simulation;

pub trait Simulate {
    fn run(&mut self);
}
