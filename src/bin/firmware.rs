#![no_main]
#![no_std]

use enigma_firmware as _; // global logger + panicking-behavior + memory layout


#[rtic::app(device = stm32h7xx_hal::pac, peripherals = true)]
mod app {
    use fugit::{Duration, Hertz, RateExtU32};
    use rtic_sync::make_signal;
    use rtic_sync::signal::{Signal, SignalReader, SignalWriter};
    
    use enigma_firmware::system::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        audio_interface: AudioInterface,
        input: Input,
        input_writer: SignalWriter<'static, InputSample>,
        input_reader: SignalReader<'static, InputSample>
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
            input_reader
        };

        input::spawn(100.Hz()).unwrap();

        defmt::trace!("Finished init");
        (Shared {}, local)
    }


    #[task(
        binds = DMA1_STR1,
        priority = 3,
        local = [
            audio_interface,
            input_reader
        ]
    )]
    fn dsp(cx: dsp::Context) {
        let dsp::LocalResources {
            audio_interface,
            input_reader,
            ..
        } = cx.local;

        audio_interface.handle_interrupt_dma1_str1(|audio_buffer| {
            for frame in audio_buffer {
                //
            }
        }).unwrap();

        if let Some(InputSample { cv1, cv2, cv3, cv4 }) = input_reader.try_read() {
            //
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