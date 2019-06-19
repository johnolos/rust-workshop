#[macro_use]
extern crate conrod;
extern crate conrod_derive;

extern crate audioengine;

mod event_loop;
mod types;
mod ui;

#[allow(unused_imports)]
use audioengine::types::KeyAction;

#[allow(unused_imports)]
use ui::Ui;

#[allow(unused_imports)]
use std::f64::consts::PI;

#[allow(unused_variables)]
fn main() -> Result<(), Error> {
    let audioengine = audioengine::EngineController::start();

    let sample_rate = audioengine.sample_rate;
    let time_per_sample = 1.0 / sample_rate;

    let mut time = 0.0;

    let mut current_key = None;

    /*
    The `move` keyword here means that values defined in the current scope are moved into whats essentially is a closure.
    The closure will be called thousands of times each second.
    You typically would want to define variables in this scope and move them inside the closure as allocation is costly.
    */
    let synth = move |action: Option<i32>| {
        time += time_per_sample;
        if action != current_key {
            current_key = action;

            println!("{:?}", action);
        }

        /*
        TODO: Your implementation of a synthesizer should be here.
        Start with returning an oscillating wave determined by the `time`-variable
        */
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
