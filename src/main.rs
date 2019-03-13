#[macro_use]
extern crate conrod;
extern crate conrod_derive;

extern crate audioengine;

mod event_loop;
mod types;
mod ui;

#[allow(unused_imports)]
use audioengine::types::{KeyAction, SignalBuffer};

#[allow(unused_imports)]
use ui::Ui;

#[allow(unused_imports)]
use std::f64::consts::PI;

const C_FREQUENCY: f64 = 261.63;
const HALFSTEP_EXP: f64 = 1.059_463_094_36;

fn transform_key_action(action: Option<i32>) -> Option<f64> {
    action.map(|v| C_FREQUENCY * HALFSTEP_EXP.powi(v))
}

#[allow(unused_variables, unused_assignments)]
fn main() -> Result<(), Error> {
    let audioengine = audioengine::EngineController::start();

    let sample_rate = audioengine.sample_rate;
    let time_per_sample = 1.0 / sample_rate;

    let mut time = 0.0;
    let mut phase = 0.0;
    let mut freq = 440.0;

    let (sender, receiver) = std::sync::mpsc::channel::<SignalBuffer>();
    let mut signal_buffer = SignalBuffer::new();

    let synth = move |action: Option<i32>| {
        time += time_per_sample;

        if let Some(new_freq) = transform_key_action(action) {
            freq = new_freq;
        }
        phase += freq * time_per_sample * 2.0 * PI;

        let mut phase_crossed_zero = false;
        if phase > PI {
            phase -= 2.0 * PI;
            phase_crossed_zero = true;
        }

        let gate = match action {
            Some(_) => 1.0,
            None => 0.0
        };

        let my_value = phase.sin() * gate;

        signal_buffer.push(my_value);

        if phase_crossed_zero {
            sender.send(signal_buffer.clone()).expect("Unable to send graph data to UI.");
            signal_buffer.clear();
        }

        my_value
    };

    audioengine.set_processor_function(Box::new(synth));

    let mut window = Ui::new(
        "Synthesizer",
        [1280.0, 800.0],
        audioengine,
        None,
        None,
        Some(receiver),
    );

    window.show();

    Ok(())
}

#[derive(Debug)]
enum Error {}
