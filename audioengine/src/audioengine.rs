extern crate portaudio;
extern crate cpal;

use portaudio::DeviceIndex;
use portaudio as pa;

use std::sync::mpsc::Sender;
use types::SignalProcessorFunction;

const SAMPLE_RATE: f64 = 44_100.0;
const FRAME_SIZE: u32 = 128;

use types::KeyAction;
use std::collections::VecDeque;
use keys_state::KeysState;

#[allow(dead_code)]
pub struct Engine {
    stream: pa::Stream<pa::NonBlocking, pa::Output<f32>>,
    stop_signal: Sender<bool>,
    gain_signal: Sender<f64>,
    key_action_signal: Sender<KeyAction>,
    signal_processor_change_sender: Sender<SignalProcessorFunction>,
}

pub struct EngineController {
    pub engine: Engine,
}

#[allow(dead_code)]
impl EngineController {
    pub fn start() -> Self {
        let portaudio = pa::PortAudio::new().unwrap();

        let output = EngineController::default_settings(&portaudio);

        let settings =
            pa::OutputStreamSettings::new(output, SAMPLE_RATE, FRAME_SIZE);

        EngineController {
            engine: Engine::start(settings, &portaudio),
        }
    }

    pub fn key_action(&mut self, action: KeyAction) {
        self.engine.key_action_signal.send(action).unwrap();
    }

    pub fn set_processor_function(&self, new_func: Box<FnMut(f64, f64, Option<i32>) -> f64>) {
        self.engine.signal_processor_change_sender.send(new_func).unwrap();
    }

    #[allow(dead_code)]
    pub fn default_settings(pa: &pa::PortAudio) -> pa::StreamParameters<f32> {
        println!("Configuring portaudio with default settings...");
        let _sample_rate = 44_100.0;
        let _frame_size = 64;

        let def_output = pa.default_output_device().unwrap();
        let output_info = pa.device_info(def_output).unwrap();
        let latency = output_info.default_low_output_latency;
        let output_params = pa::StreamParameters::<f32>::new(def_output, 2, true, latency);

        output_params
    }

}

impl Engine {
    pub fn start(
        settings: pa::OutputStreamSettings<f32>,
        portaudio: &pa::PortAudio
    ) -> Self {
        let (stop_signal, stop_slot) = ::std::sync::mpsc::channel::<bool>();
        let (gain_signal, gain_slot) = ::std::sync::mpsc::channel::<f64>();
        let (key_action_signal, key_action_slot) = ::std::sync::mpsc::channel::<KeyAction>();
        let (signal_processor_change_sender, signal_processor_change_receiver) =
            ::std::sync::mpsc::channel::<SignalProcessorFunction>();

        let mut sample_number: usize = 0;
        let sample_rate = settings.sample_rate;
        let mut gain = 0.1;

        let mut keys_pressed: [bool; 20] = [false; 20];
        let mut new_signal_fn: Option<fn(f64) -> f64> = None;

        let mut audio_processor_function: Box<FnMut(f64, f64, Option<i32>) -> f64> = Box::new(|_, _, _| 0.0);
        let mut current_key: Option<i32> = None;

        let mut keys_state = KeysState::new();

        let mut running_time = 0.0;

        let callback = move |pa::OutputStreamCallbackArgs {
                                 buffer,
                                 ..
                             }| {
            // This actually never loops says clippy
            for _msg in stop_slot.try_iter() {
                println!("Stopping audio engine.");
                return pa::Complete;
            }

            for _gain in gain_slot.try_iter() {
                gain = _gain;
            }

            for _key in key_action_slot.try_iter() {
                current_key = keys_state.key_down(_key);
            }

            for _new_processor in signal_processor_change_receiver.try_iter() {
                audio_processor_function = _new_processor;
            }

            for (_index, output_samples) in
                buffer.chunks_mut(2).enumerate()
            {
                sample_number += 1;

                let output = audio_processor_function(running_time, 1.0 / sample_rate, current_key) as f32;

                for out in output_samples {
                    *out = output;
                }

                running_time += 1.0 / sample_rate;
            }

            if let Some(_new_func) = new_signal_fn {
                new_signal_fn = None;
            }

            pa::Continue
        };

        let mut stream = portaudio
            .open_non_blocking_stream(settings, callback)
            .unwrap();

        stream.start().unwrap();

        Engine {
            stream,
            stop_signal,
            gain_signal,
            key_action_signal,
            signal_processor_change_sender: signal_processor_change_sender
        }
    }
}
