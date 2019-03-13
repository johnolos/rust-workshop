#[macro_use]
extern crate conrod;
extern crate conrod_derive;

extern crate audioengine;

mod event_loop;
mod types;
mod ui;

#[allow(unused_imports)]
use audioengine::types::{KeyAction, SignalBuffer};

use types::{GraphEvent, GraphEventType};

#[allow(unused_imports)]
use ui::Ui;

#[allow(unused_imports)]
use std::f64::consts::PI;

#[allow(unused_variables, unused_assignments)]
fn main() -> Result<(), Error> {
    let audioengine = audioengine::EngineController::start();

    let sample_rate = audioengine.sample_rate;
    let time_per_sample = 1.0 / sample_rate;

    let mut time = 0.0;
    let mut phase = 0.0;

    let (sender, receiver) = std::sync::mpsc::channel::<GraphEvent>();
    let mut signal_buffer = SignalBuffer::new();

    let synth = move |action: Option<i32>| {
        time += time_per_sample;

        let freq = 440.0;
        phase += freq * time_per_sample * 2.0 * PI;

        let mut phase_crossed_zero = false;
        if phase > PI {
            phase -= 2.0 * PI;
            phase_crossed_zero = true;
        }

        let my_value = phase.sin();

        signal_buffer.push_back(my_value);

        if phase_crossed_zero {
            sender.send((GraphEventType::SignalGraph, signal_buffer.clone(), 4410));
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
