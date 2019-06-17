#[allow(dead_code)]
pub struct Slider {
    pub min: f64,
    pub max: f64,
    pub default: f64,
    pub event_type: SliderEventType,
    pub label: String,
}

#[allow(dead_code)]
impl Slider {
    pub fn new(min: f64, max: f64, default: f64, event_type: SliderEventType, label: &str) -> Self {
        Self {
            min,
            max,
            default,
            event_type,
            label: label.to_owned(),
        }
    }
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum SliderEventType {
    Attack,
    Decay,
    Sustain,
    Release,
}

#[allow(dead_code)]
pub type SliderEvent = (SliderEventType, f64);
