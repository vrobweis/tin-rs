pub(crate) mod arc;
pub(crate) mod ellipse;
#[cfg(feature = "image")]
pub(crate) mod image;
pub(crate) mod line;
pub(crate) mod path;
pub(crate) mod rect;
pub(crate) mod state;
#[cfg(feature = "text")]
pub(crate) mod text;
pub(crate) mod triangle;

use ::nannou::prelude::*;

use crate::{
    backends::{TBackend, TinRenderer},
    color::*,
    context::{get_tin, get_tin_mut},
    event::TinEvent,
    point::TPoint,
    point::TinPoint,
    scene::TScene,
    vector2::TinVector2,
    vertex::TinVertex,
    view::TView,
    Double, Float, TColor, Tin,
};

use ::std::collections::VecDeque as Queue;

pub struct NannouBackend {
    // draw: std::sync::Mutex<Draw>,
    current_background_color: TinColor,
    pub use_layer: bool,

    path_vertices: Queue<TinVertex>,
}

impl NannouBackend {
    fn get_draw(&self) -> Draw {
        nannou::Draw::new()
    }
}

#[macro_export]
/// Macro to shorten the process of setting the draw color for a Nannou Draw instance.
macro_rules! draw_with_brush {
    ($draw:ident, $brush:ident) => {
        match $brush {
            crate::brush::TBrush::Fill(c) => {
                $draw.rgba(
                    <crate::color::TinColor as crate::color::TColor>::get_red(&c) as f32,
                    <crate::color::TinColor as crate::color::TColor>::get_green(&c) as f32,
                    <crate::color::TinColor as crate::color::TColor>::get_blue(&c) as f32,
                    <crate::color::TinColor as crate::color::TColor>::get_alpha(&c) as f32,
                );
            }
            crate::brush::TBrush::Stroke(c) => {
                $draw.stroke_color::<crate::TinColor>(c);
            }
            crate::brush::TBrush::FillAndStroke(f, s) => {
                $draw
                    .rgba(
                        <crate::color::TinColor as crate::color::TColor>::get_red(&f) as f32,
                        <crate::color::TinColor as crate::color::TColor>::get_green(&f) as f32,
                        <crate::color::TinColor as crate::color::TColor>::get_blue(&f) as f32,
                        <crate::color::TinColor as crate::color::TColor>::get_alpha(&f) as f32,
                    )
                    .stroke_color::<crate::TinColor>(s);
            }
            crate::brush::TBrush::Disabled => {
                $draw.no_fill();
            }
        };
    };
}

impl nannou::color::IntoLinSrgba<f32> for crate::TinColor {
    fn into_lin_srgba(self) -> LinSrgba<f32> {
        LinSrgba::new(
            self.get_red() as f32,
            self.get_green() as f32,
            self.get_blue() as f32,
            self.get_alpha() as f32,
        )
    }
}

impl From<Vector2> for TinPoint {
    fn from(item: Vector2) -> Self {
        TinPoint::from_coords(item.x as Double, item.y as Double)
    }
}

impl TinRenderer for NannouBackend {
    // MARK: - Drawing methods

    /** Call reset method in context that resets variables <br>
        Reset variables needed to draw:
        - buffer size should match the window size
        - fill and stroke colors
        - line width
    */
    fn prepare_for_update(&mut self) {
        eprintln!("NannouBackend::prepare_for_update()");
        self.current_background_color = DEFAULT_COLOR_BACKGROUND;
        self.use_layer = false;
        self.path_vertices.clear();
    }

    fn did_finish_update(&mut self) {
        eprintln!("NannouBackend::did_finish_update")
    }
}

impl TBackend for NannouBackend {
    fn new() -> Self {
        Self {
            // draw: Draw::default(),
            current_background_color: DEFAULT_COLOR_BACKGROUND,
            use_layer: false,

            path_vertices: Queue::<TinVertex>::new(),
        }
    }

    fn run<S>(tin_app: Tin<S>) -> Result<(), ()>
    where
        S: TScene,
    {
        eprintln!("NannouBackend::run()");
        fn model<S>(app: &App) -> S
        where
            S: TScene + 'static,
        {
            // Create a new window! Store the ID so we can refer to it later.
            let _window = app
                .new_window()
                .size(512, 512)
                .title("nannou")
                // Handle events related to the window and update the model if necessary
                .event(move |_app: &App, scene: &mut S, event: WindowEvent| {
                    // handle events
                    if let Closed = event {
                        _app.quit();
                        return;
                    }
                    let mouse_pos_point: TinPoint = TinPoint::from(_app.mouse.position());
                    get_tin_mut().mouse_moved(mouse_pos_point);
                    scene.on_event(TinEvent::from(event));
                }) // The function that will be called when the window receives events.
                .build()
                .unwrap();
            return S::setup();
        }

        // Draw the state of your `Model` into the given `Frame` here.
        fn view<S>(app: &App, _scene: &S, frame: Frame)
        where
            S: TScene,
        {
            // rendering code goes here
            let bg_color = {
                let bg_render_color = get_tin().get_background_color();
                nannou::color::lin_srgba(
                    bg_render_color.get_red() as Float,
                    bg_render_color.get_green() as Float,
                    bg_render_color.get_blue() as Float,
                    bg_render_color.get_alpha() as Float,
                )
            };
            frame.clear(bg_color);
            // Performance debugging display should be rendered here
            //get_tin_mut().frame_count = app.elapsed_frames();
            app.draw().to_frame(app, &frame).unwrap();
        }

        // Application logic here

        let tin_view = tin_app.view;
        let (frame_width, frame_height) = {
            let f = tin_view.get_frame();
            (f.get_width(), f.get_height())
        };

        nannou::app(model::<S>)
            .view(view::<S>) // The function that will be called for presenting graphics to a frame.
            .update(move |_a, scene, update| {
                let _update_time = update.since_last;
                get_tin_mut().prepare_for_update();
                scene.update();
                get_tin_mut().did_finish_update();
            })
            .size(frame_width, frame_height)
            .run();

        Ok(())
    }
}

use crate::key::TinKey;
use nannou::event::Key;
impl From<Key> for TinKey {
    fn from(k: Key) -> Self {
        match k {
            Key::Key1 => return TinKey::One,
            Key::Key2 => return TinKey::Two,
            Key::Key3 => return TinKey::Three,
            Key::Key4 => return TinKey::Four,
            Key::Key5 => return TinKey::Five,
            Key::Key6 => return TinKey::Six,
            Key::Key7 => return TinKey::Seven,
            Key::Key8 => return TinKey::Eight,
            Key::Key9 => return TinKey::Nine,
            Key::Key0 => return TinKey::Zero,
            Key::A => return TinKey::A,
            Key::B => return TinKey::B,
            Key::C => return TinKey::C,
            Key::D => return TinKey::D,
            Key::E => return TinKey::E,
            Key::F => return TinKey::F,
            Key::G => return TinKey::G,
            Key::H => return TinKey::H,
            Key::I => return TinKey::I,
            Key::J => return TinKey::J,
            Key::K => return TinKey::K,
            Key::L => return TinKey::L,
            Key::M => return TinKey::M,
            Key::N => return TinKey::N,
            Key::O => return TinKey::O,
            Key::P => return TinKey::P,
            Key::Q => return TinKey::Q,
            Key::R => return TinKey::R,
            Key::S => return TinKey::S,
            Key::T => return TinKey::T,
            Key::U => return TinKey::U,
            Key::V => return TinKey::V,
            Key::W => return TinKey::W,
            Key::X => return TinKey::X,
            Key::Y => return TinKey::Y,
            Key::Z => return TinKey::Z,
            Key::Escape => return TinKey::Escape,
            Key::F1 => return TinKey::F1,
            Key::F2 => return TinKey::F2,
            Key::F3 => return TinKey::F3,
            Key::F4 => return TinKey::F4,
            Key::F5 => return TinKey::F5,
            Key::F6 => return TinKey::F6,
            Key::F7 => return TinKey::F7,
            Key::F8 => return TinKey::F8,
            Key::F9 => return TinKey::F9,
            Key::F10 => return TinKey::F10,
            Key::F11 => return TinKey::F11,
            Key::F12 => return TinKey::F12,
            Key::F13 => return TinKey::F13,
            Key::F14 => return TinKey::F14,
            Key::F15 => return TinKey::F15,
            Key::F16 => return TinKey::F16,
            Key::F17 => return TinKey::F17,
            Key::F18 => return TinKey::F18,
            Key::F19 => return TinKey::F19,
            Key::F20 => return TinKey::F20,
            Key::F21 => return TinKey::F21,
            Key::F22 => return TinKey::F22,
            Key::F23 => return TinKey::F23,
            Key::F24 => return TinKey::F24,
            Key::Snapshot => return TinKey::Snapshot,
            Key::Scroll => return TinKey::Scroll,
            Key::Pause => return TinKey::Pause,
            Key::Insert => return TinKey::Insert,
            Key::Home => return TinKey::Home,
            Key::Delete => return TinKey::Delete,
            Key::End => return TinKey::End,
            Key::PageDown => return TinKey::PageDown,
            Key::PageUp => return TinKey::PageUp,
            Key::Left => return TinKey::Left,
            Key::Up => return TinKey::Up,
            Key::Right => return TinKey::Right,
            Key::Down => return TinKey::Down,
            Key::Back => return TinKey::Backspace,
            Key::Return => return TinKey::Enter,
            Key::Space => return TinKey::Space,
            Key::Numlock => return TinKey::NumLock,
            Key::Numpad0 => return TinKey::Zero,
            Key::Numpad1 => return TinKey::One,
            Key::Numpad2 => return TinKey::Two,
            Key::Numpad3 => return TinKey::Three,
            Key::Numpad4 => return TinKey::Four,
            Key::Numpad5 => return TinKey::Five,
            Key::Numpad6 => return TinKey::Six,
            Key::Numpad7 => return TinKey::Seven,
            Key::Numpad8 => return TinKey::Eight,
            Key::Numpad9 => return TinKey::Nine,
            Key::Apostrophe => return TinKey::Apostrophe,
            Key::Backslash => return TinKey::Backslash,
            Key::Colon => return TinKey::Colon,
            Key::Comma => return TinKey::Comma,
            Key::Equals => return TinKey::Equals,
            Key::LAlt => return TinKey::LAlt,
            Key::LBracket => return TinKey::LBracket,
            Key::LControl => return TinKey::LControl,
            Key::LShift => return TinKey::LShift,
            Key::Minus => return TinKey::Minus,
            Key::Period => return TinKey::Period,
            Key::RAlt => return TinKey::RAlt,
            Key::RBracket => return TinKey::RBracket,
            Key::RControl => return TinKey::RControl,
            Key::RShift => return TinKey::RShift,
            Key::Semicolon => return TinKey::Semicolon,
            Key::Slash => return TinKey::Slash,
            Key::Tab => return TinKey::Tab,
            Key::Grave => return TinKey::GraveAccent,
            Key::NumpadAdd => return TinKey::NumpadAdd,
            Key::NumpadDivide => return TinKey::NumpadDivide,
            Key::NumpadDecimal => return TinKey::NumpadDecimal,
            Key::NumpadEnter => return TinKey::NumpadEnter,
            Key::NumpadEquals => return TinKey::NumpadEquals,
            Key::NumpadMultiply => return TinKey::NumpadMultiply,
            Key::NumpadSubtract => return TinKey::NumpadSubtract,
            _ => TinKey::Unknown,
        }
    }
}

impl From<WindowEvent> for TinEvent {
    fn from(e: WindowEvent) -> Self {
        match e {
            Moved(p) => Self::MouseMoved(TinPoint::from(p)),
            KeyPressed(k) => Self::KeyDown(TinKey::from(k)),
            KeyReleased(k) => Self::KeyUp(TinKey::from(k)),
            MouseMoved(v) => TinEvent::MouseMoved(TinPoint::from(v)),
            MousePressed(b) => match b {
                MouseButton::Left => TinEvent::MouseDown,
                MouseButton::Right => TinEvent::RightMouseDown,
                _ => TinEvent::OtherMouseDown,
            },
            MouseReleased(b) => match b {
                MouseButton::Left => TinEvent::MouseUp,
                MouseButton::Right => TinEvent::RightMouseUp,
                _ => TinEvent::OtherMouseUp,
            },
            _ => TinEvent::Unknown,
        }
    }
}

impl From<Vector2> for TinVector2 {
    fn from(v: Vector2) -> Self {
        Self::from_xy(v.x as f64, v.y as f64)
    }
}

impl From<TinVector2> for Vector2 {
    fn from(v: TinVector2) -> Self {
        Self::new(v.x as f32, v.y as f32)
    }
}
