use std::collections::VecDeque;

pub type Phase = f64;
pub type Signal = f64;

pub type SignalProcessorFunction = Box<FnMut(Option<i32>) -> Signal + Send>;

#[derive(Clone, Copy)]
pub enum KeyAction {
    Press(i32),
    Release(i32),
}

pub type SignalFrame = Vec<Signal>;
pub type SignalBuffer = Vec<Signal>;
