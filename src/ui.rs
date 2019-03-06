extern crate conrod;

use audioengine::types::SignalBuffer;
use audioengine::EngineController;
use event_loop;
use std::path::Path;
use std::sync::mpsc::Sender;
use types::{GraphEvent, GraphEventType, Slider, SliderEvent};

use conrod::color;
use std::sync::mpsc::Receiver;

const SIGNAL_PLOT_HEIGHT: f64 = 300.0;

widget_ids! {
    struct Ids {
        // An ID for the background widget
        background,
        signal_plot_1,
        signal_plot_2,
        signal_plot_background,

        sliders[],
        slider_texts[],
    }
}

pub struct Ui<'a> {
    dimensions: [f64; 2],
    events_loop: conrod::glium::glutin::EventsLoop,
    event_loop: event_loop::EventLoop,
    display: conrod::glium::Display,
    ui: conrod::Ui,
    image_map: conrod::image::Map<conrod::glium::texture::Texture2d>,
    ids: Ids,
    sliders: &'a [Slider],
    renderer: conrod::backend::glium::Renderer,
    audioengine: EngineController,
    slider_tx: Option<Sender<SliderEvent>>,
    graphdata_rx: Option<Receiver<GraphEvent>>,
    signal_buffer: SignalBuffer,
    fft_buffer: SignalBuffer,
}

impl<'a> Ui<'a> {
    pub fn new(
        title: &str,
        dimensions: [f64; 2],
        audioengine: EngineController,
        sliders: Option<&'a [Slider]>,
        slider_tx: Option<Sender<SliderEvent>>,
        graphdata_rx: Option<Receiver<GraphEvent>>,
    ) -> Self {
        let signal_buffer: SignalBuffer = (0..2048).map(|_| 0.0).collect();
        let fft_buffer: SignalBuffer = (0..2048).map(|_| 0.0).collect();
        use conrod::glium;

        let events_loop = glium::glutin::EventsLoop::new();
        let event_loop = event_loop::EventLoop::new();
        let window = glium::glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions((dimensions[0], dimensions[1]).into());
        let context = glium::glutin::ContextBuilder::new()
            .with_vsync(true)
            .with_multisampling(1);

        let display = glium::Display::new(window, context, &events_loop).unwrap();
        let mut ui = conrod::UiBuilder::new([f64::from(1280), f64::from(800)]).build();
        let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

        let mut ids = Ids::new(ui.widget_id_generator());

        let sliders = sliders.unwrap_or(&[]);

        ids.sliders
            .resize(sliders.len(), &mut ui.widget_id_generator());
        ids.slider_texts
            .resize(sliders.len(), &mut ui.widget_id_generator());

        let renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

        Ui {
            dimensions,
            events_loop,
            event_loop,
            display,
            ui,
            image_map,
            ids,
            sliders,
            renderer,
            audioengine,
            slider_tx,
            graphdata_rx,
            signal_buffer,
            fft_buffer,
        }
    }

    pub fn show(&mut self) {
        let Ui {
            ref mut events_loop,
            event_loop,
            ref display,
            ui,
            ids,
            sliders,
            renderer,
            image_map,
            audioengine,
            slider_tx,
            graphdata_rx,
            ref mut signal_buffer,
            ref mut fft_buffer,
            ..
        } = self;

        let mut _gain = 1.0;

        let [width, _height] = self.dimensions;

        let mut slider_values: Vec<f64> = sliders.iter().map(|s| s.default).collect();

        let font_path = Path::new("./assets/fonts/Raleway-Light.ttf");

        ui.fonts.insert_from_file(font_path).unwrap();

        'main: loop {
            use audioengine::KeyAction;
            use conrod::glium;
            for event in event_loop.next(events_loop) {
                // Use the `winit` backend feature to convert the winit event to a conrod one.
                if let Some(event) = conrod::backend::winit::convert_event(event.clone(), display) {
                    ui.handle_event(event);
                    event_loop.needs_update();
                }

                match event {
                    glium::glutin::Event::WindowEvent { event, .. } => match event {
                        // Break from the loop upon `Escape`.
                        glium::glutin::WindowEvent::CloseRequested
                        | glium::glutin::WindowEvent::KeyboardInput {
                            input:
                                glium::glutin::KeyboardInput {
                                    virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => break 'main,
                        glium::glutin::WindowEvent::KeyboardInput {
                            input:
                                glium::glutin::KeyboardInput {
                                    virtual_keycode: Some(key),
                                    state: glium::glutin::ElementState::Pressed,
                                    ..
                                },
                            ..
                        } => {
                            let k = glium_key_to_binding(key);
                            match k {
                                Some(KeyboardInput::KeyInput(kee)) => {
                                    audioengine.key_action(KeyAction::Press(kee))
                                }
                                _ => (),
                            }
                        }
                        glium::glutin::WindowEvent::KeyboardInput {
                            input:
                                glium::glutin::KeyboardInput {
                                    virtual_keycode: Some(key),
                                    state: glium::glutin::ElementState::Released,
                                    ..
                                },
                            ..
                        } => {
                            let k = glium_key_to_binding(key);
                            if let Some(KeyboardInput::KeyInput(kee)) = k {
                                audioengine.key_action(KeyAction::Release(kee))
                            }
                        }
                        _ => (),
                    },
                    _ => (),
                }
            }

            let graphdata_rx_iter = graphdata_rx.iter().flat_map(|x| x.try_iter());

            // Check if we have incomming signal on reciever-channel and push it to our buffer
            for (event_type, signal_frame, size) in graphdata_rx_iter {
                match event_type {
                    GraphEventType::SignalGraph => {
                        for signal in signal_frame {
                            signal_buffer.push_back(signal);
                            while signal_buffer.len() > size {
                                signal_buffer.pop_front();
                            }
                        }
                    }
                    GraphEventType::FFTGraph => {
                        for signal in signal_frame {
                            fft_buffer.push_back(signal);
                            while fft_buffer.len() > size {
                                fft_buffer.pop_front();
                            }
                        }
                    }
                };
            }

            // Draw the widgets
            {
                use conrod::{
                    widget, Borderable, Colorable, Labelable, Positionable, Sizeable, Widget,
                };
                let ui = &mut ui.set_widgets();

                macro_rules! create_slider {
                    ($slider_id:ident, $bg_color:ident, $color:expr, $slider_type:expr, $wh:expr, $index: expr, $minmax:expr) => {{
                        let (width, height) = $wh;
                        let (min, max) = $minmax;
                        let old_value = slider_values[$index];
                        let label = format!("{:.*}", 2, old_value);
                        for value in widget::Slider::new(old_value, min, max)
                            .w_h(40.0, 150.0)
                            .bottom_left_with_margins_on(ids.background, width, height)
                            .color($color)
                            .border_color(color::DARK_GRAY)
                            .border(1.0)
                            .label(&label)
                            .label_color(color::BLACK)
                            .small_font(ui)
                            .set($slider_id, ui)
                        {
                            if let Some(tx) = slider_tx {
                                tx.send(($slider_type, value)).unwrap();
                            }
                            slider_values[$index] = value;
                        }
                    }};
                }

                macro_rules! create_slider_text {
                    ($slider_text_id:ident, $slider_id:ident, $color:expr, $text:expr) => {
                        widget::Text::new($text)
                            .down_from($slider_id, 10.0)
                            .align_middle_x_of($slider_id)
                            .font_size(16)
                            .color($color)
                            .set($slider_text_id, ui)
                    };
                }

                widget::Canvas::new()
                    .color(conrod::color::DARK_CHARCOAL)
                    .set(ids.background, ui);

                // signal background
                widget::Canvas::new()
                    .w_h(width, SIGNAL_PLOT_HEIGHT)
                    .top_left_of(ids.background)
                    .color(conrod::color::GRAY)
                    .set(ids.signal_plot_background, ui);

                // signal plot
                widget::PlotPath::new(0, signal_buffer.len(), -1.0, 1.0, |x| {
                    signal_buffer[x].max(-1.0).min(1.0)
                })
                .w_h(width, SIGNAL_PLOT_HEIGHT - 10.0)
                .middle_of(ids.signal_plot_background)
                .color(conrod::color::DARK_BLUE)
                .thickness(1.0)
                .set(ids.signal_plot_1, ui);

                // fft plot
                widget::PlotPath::new(0, fft_buffer.len(), 0.0, 2.0, |x| {
                    fft_buffer[x].max(0.0).min(2.0)
                })
                .w_h(width, SIGNAL_PLOT_HEIGHT - 10.0)
                .middle_of(ids.signal_plot_background)
                .color(conrod::color::DARK_RED)
                .thickness(1.0)
                .set(ids.signal_plot_2, ui);

                // Plotting sliders and their assosiated text labels

                for (idx, slider) in sliders.iter().enumerate() {
                    let slider_id = ids.sliders[idx];
                    let slider_text_id = ids.slider_texts[idx];
                    let x = (idx as f64) * 80.0 + 40.0;
                    create_slider!(
                        slider_id,
                        red,
                        conrod::color::rgb(0.75, 0.3, 0.3),
                        slider.event_type,
                        (40.0, x),
                        idx,
                        (slider.min, slider.max)
                    );
                    create_slider_text!(
                        slider_text_id,
                        slider_id,
                        color::RED,
                        slider.label.as_str()
                    );
                }
            }
            {
                use conrod::glium::Surface;
                if let Some(primitives) = ui.draw_if_changed() {
                    renderer.fill(&display, primitives, &image_map);
                    let mut target = display.draw();
                    target.clear_color(0.0, 0.0, 0.0, 1.0);
                    renderer.draw(display, &mut target, &image_map).unwrap();
                    target.finish().unwrap();
                }
            }
        }
    }
}

fn glium_key_to_binding(glium_key: conrod::glium::glutin::VirtualKeyCode) -> Option<KeyboardInput> {
    use self::KeyboardInput::*;
    use self::SettingsKey::*;
    use conrod::glium::glutin::VirtualKeyCode::*;
    match glium_key {
        A => Some(KeyInput(0)),
        W => Some(KeyInput(1)),
        S => Some(KeyInput(2)),
        E => Some(KeyInput(3)),
        D => Some(KeyInput(4)),
        F => Some(KeyInput(5)),
        T => Some(KeyInput(6)),
        G => Some(KeyInput(7)),
        Y => Some(KeyInput(8)),
        H => Some(KeyInput(9)),
        U => Some(KeyInput(10)),
        J => Some(KeyInput(11)),
        K => Some(KeyInput(12)),
        O => Some(KeyInput(13)),
        L => Some(KeyInput(14)),
        P => Some(KeyInput(15)),

        N => Some(SettingsInput(NextSignalFn)),
        Z => Some(SettingsInput(OctaveDown)),
        X => Some(SettingsInput(OctaveUp)),
        _ => None,
    }
}

enum KeyboardInput {
    KeyInput(i32),
    SettingsInput(SettingsKey),
}

pub enum SettingsKey {
    NextSignalFn,
    OctaveUp,
    OctaveDown,
}

#[allow(dead_code)]
fn settings_key_bindings(key: conrod::glium::glutin::VirtualKeyCode) -> Option<SettingsKey> {
    use conrod::glium::glutin::VirtualKeyCode::*;
    use ui::SettingsKey::*;

    match key {
        N => Some(NextSignalFn),
        _ => None,
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum UiError {}
