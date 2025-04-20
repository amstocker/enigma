#![no_std]

pub mod filter;
pub mod phasor;
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


#[cfg(test)]
mod tests {
    extern crate std;

    use dsp_test::Module;

    #[test]
    fn test_std() {
        std::println!("hello");
    }

}
