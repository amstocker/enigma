use daisy::pins::Gpio;
use stm32h7xx_hal::gpio::{
    gpioa::{PA3, PA7},
    gpioc::{PC0, PC4},
    Analog,
};
use stm32h7xx_hal::adc::{Adc, Enabled};
use stm32h7xx_hal::pac::{ADC1, ADC2};
use nb::block;

use super::debounce::Median;


const CV_LOW: f32 = 0.003;
const CV_HIGH: f32 = 0.970;

pub type Adc1 = Adc<ADC1, Enabled>;
pub type Adc2 = Adc<ADC2, Enabled>;

type AnalogFilter = Median<5>;


pub struct Input {
    adc1: Adc1,
    adc2: Adc2,
    cv1: PC0<Analog>,
    cv2: PA3<Analog>,
    cv3: PC4<Analog>,
    cv4: PA7<Analog>,
    filters: [AnalogFilter; 4]
}

#[derive(Default, Clone, Copy, defmt::Format)]
pub struct InputSample {
    pub cv1: f32,
    pub cv2: f32,
    pub cv3: f32,
    pub cv4: f32
}


impl Input {
    pub fn init(gpio: Gpio, adc1: Adc1, adc2: Adc2) -> Self {
        let filter = Median::new();
        Input {
            adc1,
            adc2,
            cv1: gpio.PIN_15.into_analog(),
            cv2: gpio.PIN_16.into_analog(),
            cv3: gpio.PIN_21.into_analog(),
            cv4: gpio.PIN_18.into_analog(),
            filters: [filter; 4]
        }
    }

    pub fn sample(&mut self) -> InputSample {
        let mut samples = InputSample::default();

        self.adc1.start_conversion(&mut self.cv1);
        self.adc2.start_conversion(&mut self.cv2);
        let cv1_sample = scale(block!(self.adc1.read_sample()).unwrap_or_default(), self.adc1.slope());
        let cv2_sample = scale(block!(self.adc2.read_sample()).unwrap_or_default(), self.adc2.slope());
        samples.cv1 = self.filters[0].insert(cv1_sample);
        samples.cv2 = self.filters[1].insert(cv2_sample);

        self.adc1.start_conversion(&mut self.cv3);
        self.adc2.start_conversion(&mut self.cv4);
        let cv3_sample = scale(block!(self.adc1.read_sample()).unwrap_or_default(), self.adc1.slope());
        let cv4_sample = scale(block!(self.adc2.read_sample()).unwrap_or_default(), self.adc2.slope());
        samples.cv3 = self.filters[2].insert(cv3_sample);
        samples.cv4 = self.filters[3].insert(cv4_sample);

        samples        
    }
}

fn scale(sample: u32, slope: u32) -> f32 {
    let sample = sample as f32;
    let slope = slope as f32;
    let actual = (slope - sample) / slope;
    let scaled = (actual - CV_LOW) / (CV_HIGH - CV_LOW);

    scaled.clamp(0.0, 1.0)
}