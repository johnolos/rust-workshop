#[macro_use]
extern crate conrod;
extern crate conrod_derive;

extern crate audioengine;

mod event_loop;
mod types;
mod ui;

use audioengine::types::{KeyAction, SignalBuffer};
use types::{GraphEvent, GraphEventType};
use ui::Ui;

#[allow(unused_imports)]
use std::f64::consts::PI;

const C_FREQUENCY: f64 = 261.63;
const HALFSTEP_EXP: f64 = 1.059_463_094_36;

fn transform_key_action(action: Option<KeyAction>) -> Option<f64> {
    match action {
        Some(KeyAction::Press(key_index)) => Some(C_FREQUENCY * HALFSTEP_EXP.powi(key_index)),
        Some(KeyAction::Release(_)) => None,
        _ => None
    }
}

fn main() -> Result<(), Error> {
    let audioengine = audioengine::EngineController::start();

    let (sender, receiver) = std::sync::mpsc::channel::<GraphEvent>();
    let mut signal_buffer = SignalBuffer::new();

    let mut phase = 0.0;
    let mut freq = 220.0;

    let mut gate = 0.0;

    let synth = move |_t: f64, _dt: f64, _action: Option<KeyAction>| {
        if let Some(new_freq) = transform_key_action(_action) {
            freq = new_freq;
        }

        if let Some(KeyAction::Press(_)) = _action {
            gate = 1.0;
        } else if let Some(KeyAction::Release(_)) = _action {
            gate = 0.0;
        }

        phase += freq * _dt * 2.0 * PI;

        let mut phase_crossed_zero = false;
        if phase > PI {
            phase -= 2.0 * PI;
            phase_crossed_zero = true;
        }

        let my_value = phase.sin();

        signal_buffer.push_back(my_value);

        if phase_crossed_zero {
            sender.send((GraphEventType::SignalGraph,
            signal_buffer.clone(),
            4410));
            signal_buffer.clear();
        }

        my_value * gate
    };

    audioengine.set_processor_function(Box::new(synth));

    let mut window = Ui::new("Ljubljana", [1280.0, 800.0], audioengine, None, None, Some(receiver));

    window.show();

    Ok(())
}

#[derive(Debug)]
enum Error {}
