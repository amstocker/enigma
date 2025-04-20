pub mod filter;
pub mod phasor;
pub mod debounce;
pub mod resonator;

pub use phasor::Phasor;


pub trait Process {
    fn process(&mut self, input: f32) -> f32;
}

#[derive(Clone, Copy)]
pub struct Chain<A, B> {
    pub first: A,
    pub then: B
}

impl<A: Process, B: Process> Process for Chain<A, B> {
    fn process(&mut self, input: f32) -> f32 {
        self.then.process(self.first.process(input))
    }
}
