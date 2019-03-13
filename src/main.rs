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

    let mut time = 0.0;
    let mut phase = 0.0;

    let mut current_key = None;
    let synth = move |action: Option<i32>| {
        time += time_per_sample;
        if action != current_key {
            current_key = action;

            println!("{:?}", action);
        }
        
        // TODO: Implement your synthesizer here
        0.0
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
