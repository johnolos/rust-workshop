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

    let synth = move |_t: f64, _dt: f64, _action: Option<KeyAction>| {
        // TODO: Implement your synthesizer here
        0.0
    };

    audioengine.set_processor_function(Box::new(synth));

    let mut window = Ui::new("Ljubljana", [1280.0, 800.0], audioengine, None, None, None);

    window.show();

    Ok(())
}

#[derive(Debug)]
enum Error {}
