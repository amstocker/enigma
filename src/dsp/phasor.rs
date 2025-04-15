use micromath::F32Ext;
use core::f32::consts::PI;


pub struct Phasor {
    freq: f32,
    phase: f32
}

impl Phasor {
    pub fn new(freq: f32) -> Self {
        Phasor {
            freq,
            phase: 0.0
        }
    }

    pub fn set_freq(&mut self, freq: f32) {
        self.freq = freq;
    }

    pub fn update(&mut self) {
        self.phase += self.freq;
        if !(self.phase < 1.0) {
            self.phase -= 1.0;
        }
    }

    pub fn sin(&self) -> f32 {
        (2.0 * PI * self.phase).sin()
    }
}