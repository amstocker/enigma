
#[cfg(feature = "dsp-test")]
fn main() {
    extern crate std;
    use dsp_test::Module as DSPModule;

    struct Through<const N: usize> {
        values: [f32; N]
    }

    impl<const N: usize> Through<N> {
        pub fn new() -> Self {
            Through { values: [0.0; N] }
        }
    }

    impl<const N: usize> DSPModule<N, N, 8192> for Through<N> {
        fn map_inputs(&mut self, input_buffer: &[f32; N]) {
            self.values.copy_from_slice(input_buffer);
        }
    
        fn map_outputs(&mut self, output_buffer: &mut [f32; N]) {
            output_buffer.copy_from_slice(&self.values);
        }
    }

    let module= Through::<2>::new();
    module.run().ok();
}

#[cfg(not(feature = "dsp-test"))]
fn main() {
    panic!("\"dsp-test\" feature not enabled.");
}