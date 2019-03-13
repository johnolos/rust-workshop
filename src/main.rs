#[macro_use]
extern crate conrod;
extern crate conrod_derive;

extern crate audioengine;

mod event_loop;
mod types;
mod ui;

use audioengine::types::KeyAction;
use ui::Ui;

#[allow(unused_imports)]
use std::f64::consts::PI;

fn main() -> Result<(), Error> {
    let audioengine = audioengine::EngineController::start();

    let sample_rate = audioengine.sample_rate;
    let time_per_sample = 1.0 / sample_rate;

    let mut phase = 0.0;

    let synth = move |action: Option<i32>| {

        let freq = 440.0;
        phase += freq * time_per_sample * 2.0 * PI;

        let mut phase_crossed_zero = false;
        if phase > PI {
            phase -= 2.0 * PI;
            phase_crossed_zero = true;
        }

        let my_value = phase.sin();

        my_value
    };

    audioengine.set_processor_function(Box::new(synth));

    let mut window = Ui::new(
        "Synthesizer",
        [1280.0, 800.0],
        audioengine,
        None,
        None,
        None,
    );

    window.show();

    Ok(())
}

#[derive(Debug)]
enum Error {}
