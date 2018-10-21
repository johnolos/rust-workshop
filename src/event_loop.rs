extern crate glium;
extern crate std;

use std::collections::VecDeque;

pub struct EventLoop {
    ui_needs_update: bool,
    last_update: std::time::Instant,
    frames: VecDeque<std::time::Instant>,
}

impl EventLoop {
    pub fn new() -> Self {
        EventLoop {
            last_update: std::time::Instant::now(),
            ui_needs_update: true,
            frames: VecDeque::new(),
        }
    }

    /// Produce an iterator yielding all available events.
    pub fn next(
        &mut self,
        events_loop: &mut glium::glutin::EventsLoop,
    ) -> Vec<glium::glutin::Event> {
        // We don't want to loop any faster than 60 FPS, so wait until it has been at least 16ms
        // since the last yield.
        let last_update = self.last_update;
        let sixteen_ms = std::time::Duration::from_millis(10);
        let now = std::time::Instant::now();
        let duration_since_last_update = now.duration_since(last_update);

        if duration_since_last_update < sixteen_ms {
            std::thread::sleep(sixteen_ms - duration_since_last_update);
        }

        // Collect all pending events.
        let mut events = Vec::new();
        events_loop.poll_events(|event| events.push(event));
        self.needs_update();

        // If there are no events and the `Ui` does not need updating, wait for the next event.
        if events.is_empty() && !self.ui_needs_update {
            events_loop.run_forever(|event| {
                events.push(event);
                glium::glutin::ControlFlow::Break
            });
        }

        self.ui_needs_update = false;
        self.last_update = std::time::Instant::now();

        // Fps counter
        let a_second_ago = now - std::time::Duration::from_millis(250);
        while self.frames.front().map_or(false, |t| *t < a_second_ago) {
            self.frames.pop_front();
        }
        self.frames.push_back(now);

        events
    }

    /// Notifies the event loop that the `Ui` requires another update whether or not there are any
    /// pending events.
    ///
    /// This is primarily used on the occasion that some part of the `Ui` is still animating and
    /// requires further updates to do so.
    pub fn needs_update(&mut self) {
        self.ui_needs_update = true;
    }
}
