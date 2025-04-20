#![no_main]
#![no_std]

use enigma_firmware as _; // global logger + panicking-behavior + memory layout


#[rtic::app(device = stm32h7xx_hal::pac, peripherals = true)]
mod app {
    use fugit::{Duration, Hertz, RateExtU32};
    use rtic_sync::make_signal;
    use rtic_sync::signal::{Signal, SignalReader, SignalWriter};
    
    use enigma_firmware::system::*;
    use enigma_firmware::dsp::resonator::Bank;


    const INPUT_SAMPLE_RATE: u32 = 1000; // Hz


    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        audio_interface: AudioInterface,
        input: Input,
        input_writer: SignalWriter<'static, InputSample>,
        input_reader: SignalReader<'static, InputSample>,
        resonator_bank: Bank<10>
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let System {
            audio_interface,
            input,
            ..
        } = System::init(cx.core, cx.device);

        let (input_writer, input_reader) = make_signal!(InputSample);
    
        let local = Local {
            audio_interface,
            input,
            input_writer,
            input_reader,
            resonator_bank: Bank::new(1e-4, 1e-1, 2.0, 0.5)
        };

        input::spawn(INPUT_SAMPLE_RATE.Hz()).unwrap();

        defmt::trace!("Finished init");
        (Shared {}, local)
    }


    #[task(
        binds = DMA1_STR1,
        priority = 3,
        local = [
            audio_interface,
            input_reader,
            resonator_bank
        ]
    )]
    fn dsp(cx: dsp::Context) {
        let dsp::LocalResources {
            audio_interface,
            input_reader,
            resonator_bank,
            ..
        } = cx.local;

        audio_interface.handle_interrupt_dma1_str1(|audio_buffer| {
            for frame in audio_buffer {
                //
            }
        }).unwrap();

        if let Some(input_sample) = input_reader.try_read() {
            let detected = resonator_bank.process(input_sample.cv1);
            defmt::debug!("input={}, detected={}, freq={}", input_sample.cv1, detected, detected.phase_diff * INPUT_SAMPLE_RATE as f32);
        }
    }

    #[task(local = [input, input_writer])]
    async fn input(cx: input::Context, sample_rate: Hertz<u32>) {
        let input::LocalResources {
            input,
            input_writer, ..
        } = cx.local;

        let delay: Duration<u32, 1, 1000> = sample_rate.into_duration();
        loop {
            let now = Mono::now();
            let input_sample = input.sample();
            input_writer.write(input_sample);
            
            Mono::delay_until(now + delay).await;
        }
    }
}
