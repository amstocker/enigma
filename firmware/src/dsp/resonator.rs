use core::f32::consts::PI;
use num_complex::Complex32;
use micromath::F32Ext;


#[derive(Default, Clone, Copy)]
pub struct Polar {
    norm_sq: f32,
    angle: f32
}

impl From<Complex32> for Polar {
    fn from(value: Complex32) -> Self {
        Polar {
            norm_sq: norm_sq(value),
            angle: angle(value)
        }
    }
}


#[derive(Clone, Copy)]
pub struct Resonator {
    gain: f32,
    pole: Complex32,
    complex_value: Complex32,
    polar_value: Polar,
    phase_diff: f32
}

impl Resonator {
    pub fn new(freq: f32, gain: f32, decay: f32) -> Self {
        Resonator {
            gain,
            pole: exp(Complex32 {
                re: -decay ,
                im: 2.0 * PI * freq
            }),
            complex_value: Complex32::default(),
            polar_value: Polar::default(),
            phase_diff: 0.0
        }
    }

    fn placeholder() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn process(&mut self, sample: f32) -> Polar {
        self.complex_value = self.gain * sample + self.pole * self.complex_value;

        let prev_phase = self.polar_value.angle;
        self.polar_value = self.complex_value.into();
        self.phase_diff = (self.polar_value.angle - prev_phase) % (2.0 * PI);

        self.polar_value
    }
}


#[derive(Default, defmt::Format)]
pub struct Detected {
    pub phase: f32,
    pub phase_diff: f32,
    pub magnitude_sq: f32
}

#[derive(Clone, Copy)]
pub struct Bank<const N: usize> {
    resonators: [Resonator; N]
}

impl<const N: usize> Bank<N> {
    pub fn new(freq_low: f32, freq_high: f32, gain: f32, decay: f32) -> Self {
        let freq_ratio = F32Ext::powf(freq_high / freq_low, 1.0 / (N as f32 - 1.0));

        let mut resonators = [Resonator::placeholder(); N];
        let mut freq = freq_low;
        for i in 0..N {
            resonators[i] = Resonator::new(freq, gain * freq, decay * freq);
            freq *= freq_ratio;
        }

        Bank {
            resonators
        }
    }

    pub fn process(&mut self, sample: f32) -> Detected {
        let mut max_norm_sq = 0.0;
        let mut max_norm_index = 0;
        for i in 0..N {
            let r = &mut self.resonators[i];
            r.process(sample);
            if r.polar_value.norm_sq > max_norm_sq {
                max_norm_sq = r.polar_value.norm_sq;
                max_norm_index = i;
            }
        }

        let mut mag = [0.0; N];
        for i in 0..N {
            mag[i] = self.resonators[i].polar_value.norm_sq;
        }
        defmt::debug!("\tmagnitudes: {}", mag);

        Detected {
            phase: self.resonators[max_norm_index].polar_value.angle,
            phase_diff: self.resonators[max_norm_index].phase_diff / (2.0 * PI),
            magnitude_sq: max_norm_sq
        }
    }
}


fn norm_sq(Complex32 { re, im }: Complex32) -> f32 {
    re * re + im * im
}

fn angle(Complex32 { re, im }: Complex32) -> f32 {
    im.atan2(re)
}

fn exp(Complex32 { re, im }: Complex32) -> Complex32 {
    let r = re.exp();
    Complex32 {
        re: r * im.cos(),
        im: r * im.sin()
    }
}