
pub(crate) mod arc;
pub(crate) mod ellipse;
pub(crate) mod image;
pub(crate) mod line;
pub(crate) mod path;
pub(crate) mod rect;
pub(crate) mod state;
pub(crate) mod text;
pub(crate) mod triangle;


use nannou::prelude::*;


use crate::{TinApp, context::get_tin_mut, event::{TEventHandler, TinEvent}, point::TPoint, scene::TScene};

use super::{
    super::{
        Double, Float, 
        color::*, 
        controller::TinController, frame::TinFrame,
        context::{
            get_tin
        },
        point::TinPoint, 
        shapes::{TinShape}, vector2::TVector2, vertex::TinVertex
    },
    TinRenderer, TBackend
};

use std::{collections::VecDeque as Queue};

const VS_STR: &str = include_str!("shaders/vertexshader.glsl");
const FS_STR: &str = include_str!("shaders/fragmentshader.glsl");


fn make_vertex(point: &impl TPoint, color: &TinColor) -> TinVertex {
    TinVertex::new_from_point_and_color(point, color)
}

fn make_shape_from_vertex_vec(vertices: Vec<TinVertex>) -> TinShape {
    TinShape::new(vertices)
}

fn make_shape_from_vector_vec(points: Vec<TVector2>, color: &TinColor) -> TinShape {
    let mut vertices = Vec::<TinVertex>::new();
    for point in points {
        let vertex = TinVertex::new_from_vector_and_color(&point, color);
        vertices.push(vertex);
    }
    return make_shape_from_vertex_vec(vertices);
}


const DEFAULT_LINE_WIDTH: Double = 0.05;

pub struct NannouBackend {
    pub shape_queue: Queue<TinShape>,

    save_state: bool,
    saved_shape_queue: Queue<TinShape>,

    line_width: Double,

    // pub delegate: TinContext,
    current_fill_color: TinColor,
    current_stroke_color: TinColor,
    current_background_color: TinColor,
    pub use_layer: bool,

    path_started: bool,
    path_vertices: Queue<TinVertex>
}

struct Model {
    // Store the window ID so we can refer to this specific window later if needed.
    _window: WindowId
}

impl From<Vector2> for TinPoint {
    fn from(item: Vector2) -> Self {
        TinPoint::new_from_coords(item.x as Double, item.y as Double)
    }
}

impl NannouBackend {
    
    fn model(app: &App) -> Model {
        // Create a new window! Store the ID so we can refer to it later.
        let _window = app
        .new_window()
        .size(512, 512)
        .title("nannou")
        .view(NannouBackend::view) // The function that will be called for presenting graphics to a frame.
        .event(NannouBackend::event) // The function that will be called when the window receives events.
        .build()
        .unwrap();
        
        Model { _window}
    }
    
    // Handle events related to the window and update the model if necessary
    fn event(_app: &App, model: &mut Model, event: WindowEvent) {
        // handle events

        let mouse_pos = _app.mouse.position();
        let mouse_pos_point: TinPoint = TinPoint::from(mouse_pos);

        let handler = model.handler;
        let scene = model.scene;
        let view = model.view;


        match event {
            Moved(_) => {}
            KeyPressed(k) => {
                let tk_option = map_key_to_tin_key(k);
                match tk_option {
                    Some(k) => TinController::on_event(TinEvent::KeyDown(k), scene),
                    None => {}
                }
            }
            KeyReleased(k) => {
                let tk_option = map_key_to_tin_key(k);
                match tk_option {
                    Some(k) => TinController::on_event(TinEvent::KeyUp(k), scene),
                    None => {}
                }
            }
            MouseMoved(v) => { TinController::on_event(TinEvent::MouseMoved(TinPoint::from(v)), scene)}
            MousePressed(b) => {
                match b {
                    MouseButton::Left => { TinController::on_event(TinEvent::MouseDown(mouse_pos_point), scene)}
                    MouseButton::Right => { TinController::on_event(TinEvent::RightMouseDown(mouse_pos_point), scene)}
                    _ => { TinController::on_event(TinEvent::OtherMouseDown(mouse_pos_point), scene)}
                }
            }
            MouseReleased(b) => {
                match b {
                    MouseButton::Left => { TinController::on_event(TinEvent::MouseUp(mouse_pos_point), scene)}
                    MouseButton::Right => { TinController::on_event(TinEvent::RightMouseUp(mouse_pos_point), scene)}
                    _ => { TinController::on_event(TinEvent::OtherMouseUp(mouse_pos_point), scene)}
                }
            }
            MouseEntered => {}
            MouseExited => {}
            MouseWheel(_, _) => {}
            Resized(_) => {}
            HoveredFile(_) => {}
            DroppedFile(_) => {}
            HoveredFileCancelled => {}
            Touch(_) => {}
            Focused => {}
            Unfocused => {}
            Closed => {_app.quit()}
            _ => {}
        }
    }
    
    // Draw the state of your `Model` into the given `Frame` here.
    fn view(app: &App, _model: &Model, frame: Frame) {
        let bg_color;
        {
            let bg_render_color;
            {
                bg_render_color = get_tin().get_background_color();
            }
            {
                bg_color = nannou::color::lin_srgba(
                    bg_render_color.red as Float, 
                    bg_render_color.green as Float, 
                    bg_render_color.blue as Float, 
                    bg_render_color.alpha as Float
                )
            }
        }
        
        frame.clear(bg_color);

        let mut tin = get_tin_mut();
        tin.frame_count = app.elapsed_frames();

        let draw = app.draw();
        

        draw.to_frame(app, &frame).unwrap();
    }

    fn enqueue_shape(&mut self, mut points: Vec<TVector2>) {
        eprintln!("NannouBackend::enqueue_shape()");

        let fill_color = get_tin().get_fill_color();
        let stroke_color = get_tin().get_stroke_color();

        let use_fill: bool = fill_color.alpha > 0.0;
        let use_stroke: bool = stroke_color.alpha > 0.0;

        for p in &mut points {
            p.rotate(get_tin().rotation)
        }
        
        let shape_queue = &mut self.shape_queue;
        if use_fill && use_stroke {
            shape_queue.push_back(make_shape_from_vector_vec(points.clone(), &fill_color));
            shape_queue.push_back(make_shape_from_vector_vec(points, &stroke_color))
        } else
        if use_fill {
            shape_queue.push_back(make_shape_from_vector_vec(points, &fill_color))
        } else
        if use_stroke {
            shape_queue.push_back(make_shape_from_vector_vec(points, &stroke_color))
        }
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

        self.shape_queue.clear();
        self.saved_shape_queue.clear();

        self.save_state = false;

        self.current_fill_color = DEFAULT_COLOR_FILL;
        self.current_stroke_color = DEFAULT_COLOR_STROKE;
        self.current_background_color = DEFAULT_COLOR_BACKGROUND;

        self.use_layer = false;

        self.path_started = false;
        self.path_vertices.clear();

        
        

        assert_eq!(self.shape_queue.len(), 0);
        assert_eq!(self.saved_shape_queue.len(), 0);
        assert_eq!(self.save_state, false);
        assert_eq!(self.current_fill_color, DEFAULT_COLOR_FILL);
        assert_eq!(self.current_stroke_color, DEFAULT_COLOR_STROKE);
        assert_eq!(self.current_background_color, DEFAULT_COLOR_BACKGROUND);
        assert_eq!(self.path_started, false);
        assert_eq!(self.path_vertices.len(), 0);
    }

    fn did_finish_update(&mut self) {
        eprintln!("NannouBackend::did_finish_update")
    }
    
}


impl TBackend for NannouBackend {

    fn new() -> Self {
        Self {
            shape_queue: Queue::<TinShape>::new(),

            save_state: false,
            saved_shape_queue: Queue::<TinShape>::new(),

            line_width: DEFAULT_LINE_WIDTH as Double,

            //delegate: TinContext::init(),// Probably need to change this when the context is fully implemented
            current_fill_color: DEFAULT_COLOR_FILL,
            current_stroke_color: DEFAULT_COLOR_STROKE,
            current_background_color: DEFAULT_COLOR_BACKGROUND,
            use_layer: false,

            path_started: false,
            path_vertices: Queue::<TinVertex>::new()
        }
    }

    fn run<S>(mut app: TinApp<S>) -> Result<(),()> where S: TScene {
        eprintln!("NannouBackend::run()");
        // Application logic here

        let controller = app.controller;

        nannou::app(NannouBackend::model)
        .run();

        {
            get_tin_mut().prepare_for_update();
        }
        app.scene.update();
        
        // Performance debugging display should be rendered here
        {
            get_tin_mut().did_finish_update();
        }
        
        
        // rendering code goes here
        
        Ok(())
    }
}

use nannou::event::Key;
use crate::key::TinKey;

fn map_key_to_tin_key(k: Key) -> Option<TinKey> {
    match k {
        Key::Key1 => {return Some(TinKey::One)}
        Key::Key2 => {return Some(TinKey::Two)}
        Key::Key3 => {return Some(TinKey::Three)}
        Key::Key4 => {return Some(TinKey::Four)}
        Key::Key5 => {return Some(TinKey::Five)}
        Key::Key6 => {return Some(TinKey::Six)}
        Key::Key7 => {return Some(TinKey::Seven)}
        Key::Key8 => {return Some(TinKey::Eight)}
        Key::Key9 => {return Some(TinKey::Nine)}
        Key::Key0 => {return Some(TinKey::Zero)}
        Key::A => {return Some(TinKey::A)}
        Key::B => {return Some(TinKey::B)}
        Key::C => {return Some(TinKey::C)}
        Key::D => {return Some(TinKey::D)}
        Key::E => {return Some(TinKey::E)}
        Key::F => {return Some(TinKey::F)}
        Key::G => {return Some(TinKey::G)}
        Key::H => {return Some(TinKey::H)}
        Key::I => {return Some(TinKey::I)}
        Key::J => {return Some(TinKey::J)}
        Key::K => {return Some(TinKey::K)}
        Key::L => {return Some(TinKey::L)}
        Key::M => {return Some(TinKey::M)}
        Key::N => {return Some(TinKey::N)}
        Key::O => {return Some(TinKey::O)}
        Key::P => {return Some(TinKey::P)}
        Key::Q => {return Some(TinKey::Q)}
        Key::R => {return Some(TinKey::R)}
        Key::S => {return Some(TinKey::S)}
        Key::T => {return Some(TinKey::T)}
        Key::U => {return Some(TinKey::U)}
        Key::V => {return Some(TinKey::V)}
        Key::W => {return Some(TinKey::W)}
        Key::X => {return Some(TinKey::X)}
        Key::Y => {return Some(TinKey::Y)}
        Key::Z => {return Some(TinKey::Z)}
        Key::Escape => {return Some(TinKey::Escape)}
        Key::F1 => {return Some(TinKey::F1)}
        Key::F2 => {return Some(TinKey::F2)}
        Key::F3 => {return Some(TinKey::F3)}
        Key::F4 => {return Some(TinKey::F4)}
        Key::F5 => {return Some(TinKey::F5)}
        Key::F6 => {return Some(TinKey::F6)}
        Key::F7 => {return Some(TinKey::F7)}
        Key::F8 => {return Some(TinKey::F8)}
        Key::F9 => {return Some(TinKey::F9)}
        Key::F10 => {return Some(TinKey::F10)}
        Key::F11 => {return Some(TinKey::F11)}
        Key::F12 => {return Some(TinKey::F12)}
        Key::F13 => {return Some(TinKey::F13)}
        Key::F14 => {return Some(TinKey::F14)}
        Key::F15 => {return Some(TinKey::F15)}
        Key::F16 => {return Some(TinKey::F16)}
        Key::F17 => {return Some(TinKey::F17)}
        Key::F18 => {return Some(TinKey::F18)}
        Key::F19 => {return Some(TinKey::F19)}
        Key::F20 => {return Some(TinKey::F20)}
        Key::F21 => {return Some(TinKey::F21)}
        Key::F22 => {return Some(TinKey::F22)}
        Key::F23 => {return Some(TinKey::F23)}
        Key::F24 => {return Some(TinKey::F24)}
        Key::Snapshot => {return Some(TinKey::Snapshot)}
        Key::Scroll => {return Some(TinKey::Scroll)}
        Key::Pause => {return Some(TinKey::Pause)}
        Key::Insert => {return Some(TinKey::Insert)}
        Key::Home => {return Some(TinKey::Home)}
        Key::Delete => {return Some(TinKey::Delete)}
        Key::End => {return Some(TinKey::End)}
        Key::PageDown => {return Some(TinKey::PageDown)}
        Key::PageUp => {return Some(TinKey::PageUp)}
        Key::Left => {return Some(TinKey::Left)}
        Key::Up => {return Some(TinKey::Up)}
        Key::Right => {return Some(TinKey::Right)}
        Key::Down => {return Some(TinKey::Down)}
        Key::Back => {return Some(TinKey::Backspace)}
        Key::Return => {return Some(TinKey::Enter)}
        Key::Space => {return Some(TinKey::Space)}
        Key::Numlock => {return Some(TinKey::NumLock)}
        Key::Numpad0 => {return Some(TinKey::Zero)}
        Key::Numpad1 => {return Some(TinKey::One)}
        Key::Numpad2 => {return Some(TinKey::Two)}
        Key::Numpad3 => {return Some(TinKey::Three)}
        Key::Numpad4 => {return Some(TinKey::Four)}
        Key::Numpad5 => {return Some(TinKey::Five)}
        Key::Numpad6 => {return Some(TinKey::Six)}
        Key::Numpad7 => {return Some(TinKey::Seven)}
        Key::Numpad8 => {return Some(TinKey::Eight)}
        Key::Numpad9 => {return Some(TinKey::Nine)}
        Key::Apostrophe => {return Some(TinKey::Apostrophe)}
        Key::Backslash => {return Some(TinKey::Backslash)}
        Key::Colon => {return Some(TinKey::Colon)}
        Key::Comma => {return Some(TinKey::Comma)}
        Key::Equals => {return Some(TinKey::Equals)}
        Key::LAlt => {return Some(TinKey::LAlt)}
        Key::LBracket => {return Some(TinKey::LBracket)}
        Key::LControl => {return Some(TinKey::LControl)}
        Key::LShift => {return Some(TinKey::LShift)}
        Key::Minus => {return Some(TinKey::Minus)}
        Key::Period => {return Some(TinKey::Period)}
        Key::RAlt => {return Some(TinKey::RAlt)}
        Key::RBracket => {return Some(TinKey::RBracket)}
        Key::RControl => {return Some(TinKey::RControl)}
        Key::RShift => {return Some(TinKey::RShift)}
        Key::Semicolon => {return Some(TinKey::Semicolon)}
        Key::Slash => {return Some(TinKey::Slash)}
        Key::Tab => {return Some(TinKey::Tab)}
        Key::Grave => {return Some(TinKey::GraveAccent)}
        Key::NumpadAdd => {return Some(TinKey::NumpadAdd)}
        Key::NumpadDivide => {return Some(TinKey::NumpadDivide)}
        Key::NumpadDecimal => {return Some(TinKey::NumpadDecimal)}
        Key::NumpadEnter => {return Some(TinKey::NumpadEnter)}
        Key::NumpadEquals => {return Some(TinKey::NumpadEquals)}
        Key::NumpadMultiply => {return Some(TinKey::NumpadMultiply)}
        Key::NumpadSubtract => {return Some(TinKey::NumpadSubtract)}
        _ => None
    }
}