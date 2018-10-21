extern crate portaudio;
use portaudio::DeviceIndex;
use portaudio as pa;

use std::sync::mpsc::Sender;
use types::SignalProcessorFunction;

const SAMPLE_RATE: f64 = 44_100.0;
const FRAME_SIZE: u32 = 128;

use types::KeyAction;

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
    pub output: pa::StreamParameters<f32>,
    pub portaudio: pa::PortAudio
}

#[allow(dead_code)]
impl EngineController {
    pub fn start() -> Self {
        let portaudio = pa::PortAudio::new().unwrap();
        #[cfg(asio)]
        let output = EngineController::asio_settings(&portaudio);

        #[cfg(not(asio))]
        let output = EngineController::default_settings(&portaudio);

        let settings =
            pa::OutputStreamSettings::new(output, SAMPLE_RATE, FRAME_SIZE);

        EngineController {
            engine: Engine::start(settings, &portaudio),
            output,
            portaudio
        }
    }

    #[allow(dead_code)]
    pub fn replace(
        &mut self
    ) {
        self.stop();
        let settings =
            pa::OutputStreamSettings::new(self.output, SAMPLE_RATE, FRAME_SIZE);
        self.engine = Engine::start(settings, &self.portaudio);
    }

    pub fn stop(&mut self) {
        self.engine.stream.stop().unwrap();
        println!("{:#?}", self.engine.stop_signal.send(true));
    }

    pub fn set_gain(&mut self, new_gain: f64) {
        self.engine.gain_signal.send(new_gain).unwrap();
    }

    pub fn key_action(&mut self, action: KeyAction) {
        self.engine.key_action_signal.send(action).unwrap();
    }

    pub fn set_processor_function(&self, new_func: Box<FnMut(f64, f64, Option<KeyAction>) -> f64>) {
        self.engine.signal_processor_change_sender.send(new_func).unwrap();
    }

    pub fn set_output_device(&mut self, output_device: DeviceIndex) {
        let output_info = self.portaudio.device_info(output_device).unwrap();
        let latency = output_info.default_low_output_latency;
        self.output = pa::StreamParameters::<f32>::new(output_device, 2, true, latency);
    }

    pub fn set_input_device(&mut self, input_device: DeviceIndex) {
        let input_info = self.portaudio.device_info(input_device).unwrap();
        let latency = input_info.default_low_input_latency;
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

    #[allow(dead_code)]
    pub fn asio_settings(pa: &pa::PortAudio) -> pa::StreamParameters<f32> {
        println!("Configuring portaudio with ASIO...");
        let sample_rate = 44_100.0;
        let frame_size = 128;

        let asio_host_api = pa
            .host_apis()
            .find(|(_idx, host)| host.name == "ASIO")
            .map(|(_idx, host)| host);

        let output_device = asio_host_api
            .clone()
            .and_then(|api| api.default_output_device);
        let output_info = output_device.and_then(|od| pa.device_info(od).ok());
        let output_params = output_device
            .iter()
            .zip(output_info.iter())
            .map(|(&od, oi)| {
                pa::StreamParameters::<f32>::new(
                    od,
                    oi.max_output_channels,
                    true,
                    oi.default_low_input_latency,
                )
            })
            .next();

        output_params.unwrap()
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

        let mut audio_processor_function: Box<FnMut(f64, f64, Option<KeyAction>) -> f64> = Box::new(|_, _, _| 0.0);
        let mut current_key: Option<KeyAction> = None;

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
                current_key = Some(_key);
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
