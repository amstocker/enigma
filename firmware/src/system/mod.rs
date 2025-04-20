pub mod cv;
pub mod debounce;

use stm32h7xx_hal::delay::DelayFromCountDownTimer;
use stm32h7xx_hal::prelude::*;
use stm32h7xx_hal::pac::CorePeripherals;
use stm32h7xx_hal::pac::Peripherals as DevicePeripherals;
use stm32h7xx_hal::adc;
use stm32h7xx_hal::rng::Rng;

pub use rtic_monotonics::systick::prelude::*;
pub use daisy::audio::Interface as AudioInterface;

pub use crate::system::cv::{Input, InputSample};


systick_monotonic!(Mono, 1000);

pub struct System {
    pub audio_interface: AudioInterface,
    pub input: Input,
    pub random: Random
}

pub struct Random(Rng);

impl Random {
    pub fn float(&mut self) -> f32 {
        let x: Result<u32, _> = self.0.r#gen();
        
        (x.unwrap() as f32 / core::u32::MAX as f32) % 1.0
    }
}

impl System {
    pub fn init(mut cp: CorePeripherals, dp: DevicePeripherals) -> Self {
        cp.SCB.enable_icache();
        cp.SCB.enable_dcache(&mut cp.CPUID);

        let board = daisy::Board::take().unwrap();
        let ccdr = daisy::board_freeze_clocks!(board, dp);
        let pins = daisy::board_split_gpios!(board, ccdr, dp);

        Mono::start(cp.SYST, ccdr.clocks.sysclk().to_Hz());

        let random = Random(dp.RNG.constrain(ccdr.peripheral.RNG, &ccdr.clocks));

        let audio_interface = daisy::board_split_audio!(ccdr, pins)
            .spawn()
            .unwrap();

        let mut delay = DelayFromCountDownTimer::new(dp.TIM2.timer(
            100.Hz(),
            ccdr.peripheral.TIM2,
            &ccdr.clocks,
        ));
        
        let (adc1, adc2) = adc::adc12(
            dp.ADC1,
            dp.ADC2,
            4.MHz(),
            &mut delay,
            ccdr.peripheral.ADC12,
            &ccdr.clocks,
        );

        let mut adc1 = adc1.enable();
        adc1.set_resolution(adc::Resolution::SixteenBit);
        adc1.set_sample_time(adc::AdcSampleTime::T_16);

        let mut adc2 = adc2.enable();
        adc2.set_resolution(adc::Resolution::SixteenBit);
        adc2.set_sample_time(adc::AdcSampleTime::T_16);

        let input = Input::init(pins.GPIO, adc1, adc2);

        System {
            audio_interface,
            input,
            random
        }
    }
}