use super::Process;


// FREQ_TO_DECAY = (2 * pi) / sqrt(2^(1/4) - 1)
const FREQ_TO_DECAY: f32 = 14.444786810209614;
const FUDGE: f32 = 0.3;


#[derive(Clone, Copy)]
pub struct LowPassFilter {
    decay: f32,
    input_buffer: f32,
    output_buffer: f32
}

impl LowPassFilter {
    pub fn new(decay: f32, init: f32) -> Self {
        LowPassFilter {
            decay,
            input_buffer: 0.0,
            output_buffer: init
        }
    }

    pub fn set_cutoff(&mut self, freq: f32) {
        let decay = FREQ_TO_DECAY * FUDGE * freq;
        self.set_decay(decay);
    }

    pub fn set_decay(&mut self, decay: f32) {
        self.decay = decay;
    }
}

impl Process for LowPassFilter {
    fn process(&mut self, input: f32) -> f32 {
        self.output_buffer += self.decay * (0.5 * input + 0.5 * self.input_buffer - self.output_buffer);
        self.input_buffer = input;
        self.output_buffer
    }
}


#[derive(Clone, Copy)]
pub struct NPoleLPF<const N: usize> {
    stages: [LowPassFilter; N]
}

impl<const N: usize> NPoleLPF<N> {
    pub fn new(decay: f32, init: f32) -> Self {
        NPoleLPF {
            stages: [LowPassFilter::new(decay, init); N]
        }
    }

    pub fn set_cutoff(&mut self, freq: f32) {
        for i in 0..N {
            self.stages[i].set_cutoff(freq);
        }
    }

    pub fn set_decay(&mut self, decay: f32) {
        for i in 0..N {
            self.stages[i].set_decay(decay);
        }
    }

    pub fn get_decay(&self) -> f32 {
        self.stages[0].decay
    }
}

impl<const N: usize> Process for NPoleLPF<N> {
    fn process(&mut self, input: f32) -> f32 {
        let mut output = input;
        for i in 0..N {
            output = self.stages[i].process(output);
        }
        output
    }
}


#[derive(Clone, Copy)]
pub struct AutoGain {
    lpf: LowPassFilter
}

impl AutoGain {
    pub fn new(decay: f32) -> Self {
        AutoGain {
            lpf: LowPassFilter::new(decay, 1.0)
        }
    }

    pub fn process(&mut self, input: f32) -> f32 {
        let amp = self.lpf.process(input.abs());
        let output = (1.0 / amp) * input;
        
        output
    }
}


pub struct HighPassFilter {
    decay: f32,
    input_buffer: f32,
    output_buffer: f32
}

impl HighPassFilter {
    pub fn new(decay: f32, init: f32) -> Self {
        HighPassFilter {
            decay,
            input_buffer: 0.0,
            output_buffer: init
        }
    }

    pub fn set_cutoff(&mut self, freq: f32) {
        let decay = FREQ_TO_DECAY * FUDGE * freq;
        self.set_decay(decay);
    }

    pub fn set_decay(&mut self, decay: f32) {
        self.decay = decay;
    }

    pub fn process(&mut self, input: f32) -> f32 {
        self.output_buffer += self.decay * (0.5 * input - 0.5 * self.input_buffer - self.output_buffer);
        self.input_buffer = input;
        self.output_buffer
    }
}
