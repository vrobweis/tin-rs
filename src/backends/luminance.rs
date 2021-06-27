pub(crate) mod arc;
pub(crate) mod ellipse;
#[cfg(image)]
pub(crate) mod image;
pub(crate) mod line;
pub(crate) mod path;
pub(crate) mod rect;
pub(crate) mod state;
#[cfg(text)]
pub(crate) mod text;
pub(crate) mod triangle;

use glfw::{Action, Context, WindowEvent};
use luminance::{
    context::GraphicsContext,
    pipeline::PipelineState,
    render_state::RenderState,
    tess::{Mode, Tess},
};
use luminance_gl::gl33::GL33;
use luminance_glfw::{GL33Context, GlfwSurface};
use luminance_windowing::{WindowDim, WindowOpt};
use std::{collections::VecDeque as Queue, time::Instant};

use crate::*;
use crate::{
    backends::{TBackend, TinRenderer},
    brush::TBrush,
    color::TColor,
    context::{get_tin, get_tin_mut, DrawState},
    event::TinEvent,
    point::{TPoint, TinPoint},
    scene::TScene,
    shapes::*,
    vector2::TinVector2,
    vertex::*,
    view::TView,
};
pub(crate) struct LuminanceBackend {
    pub shape_queue: Queue<TinShape>,

    path_started: bool,
    path_vertices: Queue<TinVertex>,
}

impl LuminanceBackend {
    fn enqueue_shape(&mut self, mut points: Vec<TinVector2>, brush: TBrush, state: DrawState) {
        for p in &mut points {
            p.rotate(state.rotation)
        }

        let shape_queue = &mut self.shape_queue;
        match brush {
            TBrush::Fill(c) => shape_queue.push_back(make_shape_from_vector_vec(points, &c)),
            TBrush::Stroke(c) => shape_queue.push_back(make_shape_from_vector_vec(points, &c)),
            TBrush::FillAndStroke(f, s) => {
                shape_queue.push_back(make_shape_from_vector_vec(points.clone(), &f));
                shape_queue.push_back(make_shape_from_vector_vec(points, &s))
            }
            TBrush::Disabled => {}
        }
    }
}

impl TinRenderer for LuminanceBackend {
    // MARK: - Drawing methods

    /** Call reset method in context that resets variables <br>
        Reset variables needed to draw:
        - buffer size should match the window size
        - fill and stroke colors
        - line width
    */
    fn prepare_for_update(&mut self) {
        eprintln!("LuminanceBackend::prepare_for_update()");

        self.shape_queue.clear();

        self.path_started = false;
        self.path_vertices.clear();

        assert_eq!(self.shape_queue.len(), 0);

        assert_eq!(self.path_started, false);
        assert_eq!(self.path_vertices.len(), 0);
    }

    fn did_finish_update(&mut self) {
        eprintln!("LuminanceBackend::did_finish_update")
    }
}

impl TBackend for LuminanceBackend {
    fn new() -> Self {
        Self {
            shape_queue: Queue::<TinShape>::new(),

            //delegate: TinContext::init(),// Probably need to change this when the context is fully implemented
            path_started: false,
            path_vertices: Queue::<TinVertex>::new(),
        }
    }

    fn run<S>(app: Tin<S>) -> Result<(), ()>
    where
        S: TScene,
    {
        eprintln!("LuminanceBackend::run()");
        // Application logic here
        let view = &app.view;

        let mut surface = produce_graphics_surface(view);

        let ctxt = &mut surface.context;
        let events = &surface.events_rx;

        // This being mutable lets us resize it.
        let mut back_buffer = ctxt.back_buffer().expect("Could not get back buffer");

        // Includes the TinVertexSemantics generic type so the shader program can be checked against the Tess it's used with,
        // so vertex definitions are consistent
        let mut program = ctxt
            .new_shader_program::<TinVertexSemantics, (), ()>()
            .from_strings(backends::VS_STR, None, None, backends::FS_STR)
            .unwrap()
            .ignore_warnings();

        let mut last_frame_time = Instant::now();

        let mut scene = S::setup();

        'apploop: loop {
            {
                get_tin_mut().prepare_for_update();
            }
            scene.update();

            // Performance debugging display should be rendered here
            {
                let mut tin = get_tin_mut();
                tin.process_draw_calls();
                tin.did_finish_update();
            }

            // handle events
            ctxt.window.glfw.poll_events(); // Fill receiver with events
            for (_, event) in glfw::flush_messages(&events) {
                match event {
                    // End loop
                    WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                        break 'apploop
                    }
                    WindowEvent::Char(_) => {}
                    WindowEvent::CharModifiers(_, _) => {}
                    WindowEvent::FramebufferSize(..) => back_buffer = ctxt.back_buffer().unwrap(),

                    WindowEvent::Key(k, _, Action::Press, _) => {
                        scene.on_event(TinEvent::KeyDown(TinKey::from(k)))
                    }
                    WindowEvent::Key(k, _, Action::Release, _) => {
                        scene.on_event(TinEvent::KeyUp(TinKey::from(k)))
                    }
                    WindowEvent::Key(_k, _, _, _) => {}

                    WindowEvent::Pos(_, _) => {}
                    WindowEvent::Size(_, _) => {}
                    WindowEvent::Refresh => {}
                    WindowEvent::Focus(_) => {}
                    WindowEvent::Iconify(_) => {}

                    WindowEvent::MouseButton(glfw::MouseButton::Button1, Action::Press, _) => {
                        scene.on_event(TinEvent::MouseDown)
                    }
                    WindowEvent::MouseButton(glfw::MouseButton::Button1, Action::Release, _) => {
                        scene.on_event(TinEvent::MouseUp)
                    }
                    WindowEvent::MouseButton(glfw::MouseButton::Button1, Action::Repeat, _) => {
                        scene.on_event(TinEvent::MouseDown)
                    }

                    WindowEvent::MouseButton(glfw::MouseButton::Button2, Action::Press, _) => {
                        scene.on_event(TinEvent::RightMouseDown)
                    }
                    WindowEvent::MouseButton(glfw::MouseButton::Button2, Action::Release, _) => {
                        scene.on_event(TinEvent::RightMouseUp)
                    }
                    WindowEvent::MouseButton(glfw::MouseButton::Button2, Action::Repeat, _) => {
                        scene.on_event(TinEvent::RightMouseDown)
                    }

                    WindowEvent::MouseButton(_, Action::Press, _) => {
                        scene.on_event(TinEvent::OtherMouseDown)
                    }
                    WindowEvent::MouseButton(_, Action::Release, _) => {
                        scene.on_event(TinEvent::OtherMouseUp)
                    }
                    WindowEvent::MouseButton(_, Action::Repeat, _) => {
                        scene.on_event(TinEvent::OtherMouseDown)
                    }

                    WindowEvent::CursorPos(x, y) => {
                        let mouse_pos = TinPoint::from_coords(x, y);
                        get_tin_mut().mouse_moved(mouse_pos.clone());
                        scene.on_event(TinEvent::MouseMoved(mouse_pos))
                    }
                    WindowEvent::CursorEnter(_) => {}
                    WindowEvent::Scroll(_, _) => {}
                    WindowEvent::FileDrop(_) => {}
                    WindowEvent::Maximize(_) => {}
                    WindowEvent::ContentScale(_, _) => {}
                }
            }

            let mut tesses: Queue<Tess<GL33, TinVertex>> = prepare_shapes_for_render(ctxt);

            // rendering code goes here

            let bg_color;
            {
                let bg_render_color = get_tin().get_background_color();
                bg_color = [
                    bg_render_color.get_red() as Float,
                    bg_render_color.get_green() as Float,
                    bg_render_color.get_blue() as Float,
                    bg_render_color.get_alpha() as Float,
                ]
            }

            let pipeline_state = PipelineState::default()
                .set_clear_color(bg_color)
                .enable_clear_color(true);

            let render = ctxt
                .new_pipeline_gate()
                .pipeline(
                    // the frame buffer to write to
                    &back_buffer,
                    // object containing color to use when clearing framebuffer color buffers
                    &pipeline_state,
                    |_, mut shd_gate| {
                        // Start shading
                        shd_gate.shade(&mut program, |_, _, mut rdr_gate| {
                            // Start rendering
                            rdr_gate.render(&RenderState::default(), |mut tess_gate| {
                                while !tesses.is_empty() {
                                    let shape = &tesses.pop_front().unwrap();
                                    tess_gate.render(shape)?;
                                }
                                Ok(())
                            })
                        })
                    },
                )
                .assume();

            // swap buffer chains and draw the back buffer to the front buffer, thus displaying to user
            if render.is_ok() {
                let fps = app.get_fps();
                crate::stopwatch::pace_frames(fps, last_frame_time);
                ctxt.window.swap_buffers();
                last_frame_time = Instant::now();
            } else {
                break 'apploop;
            }
        }
        Ok(())
    }
}

fn produce_graphics_surface(view_ref: &impl TView) -> GlfwSurface {
    let view_frame = view_ref.get_frame();

    let view_title = view_ref.get_title();

    let win_opt = WindowOpt::default()
        .set_dim(WindowDim::Windowed {
            width: view_frame.get_width(),
            height: view_frame.get_height(),
        })
        .set_cursor_mode(luminance_windowing::CursorMode::Visible);

    GlfwSurface::new_gl33(view_title, win_opt).expect("GlfwSurface could not be built")
}

fn prepare_shapes_for_render(context: &mut GL33Context) -> Queue<Tess<GL33, TinVertex>> {
    let mut tesses: Queue<Tess<GL33, TinVertex>> = Queue::new();
    let shapes = &mut get_tin_mut().render.shape_queue;
    while !shapes.is_empty() {
        let shape = shapes.pop_front().unwrap();

        let vertex_count = shape.get_vertices().len();
        let draw_mode: Mode;
        match vertex_count {
            1 => panic!("Shape had one vertex"),
            2 => panic!("Shape had two vertices"), // draw_mode = Mode::Line,
            3 => draw_mode = Mode::Triangle,
            5 => draw_mode = Mode::TriangleStrip,
            _ => draw_mode = Mode::Patch(vertex_count),
        }

        let shape_tess = context
            .new_tess()
            .set_vertices(shape)
            .set_mode(draw_mode)
            .build()
            .expect("Could not build tesselation.");
        tesses.push_back(shape_tess);
    }
    tesses
}

use glfw::Key;
impl From<glfw::Key> for TinKey {
    fn from(k: glfw::Key) -> Self {
        match k {
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
            Key::Num1 => return TinKey::One,
            Key::Num2 => return TinKey::Two,
            Key::Num3 => return TinKey::Three,
            Key::Num4 => return TinKey::Four,
            Key::Num5 => return TinKey::Five,
            Key::Num6 => return TinKey::Six,
            Key::Num7 => return TinKey::Seven,
            Key::Num8 => return TinKey::Eight,
            Key::Num9 => return TinKey::Nine,
            Key::Num0 => return TinKey::Zero,
            Key::Kp0 => return TinKey::Zero,
            Key::Kp1 => return TinKey::One,
            Key::Kp2 => return TinKey::Two,
            Key::Kp3 => return TinKey::Three,
            Key::Kp4 => return TinKey::Four,
            Key::Kp5 => return TinKey::Five,
            Key::Kp6 => return TinKey::Six,
            Key::Kp7 => return TinKey::Seven,
            Key::Kp8 => return TinKey::Eight,
            Key::Kp9 => return TinKey::Nine,
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
            Key::PrintScreen => return TinKey::Snapshot,
            Key::ScrollLock => return TinKey::Scroll,
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
            Key::Backspace => return TinKey::Backspace,
            Key::Enter => return TinKey::Enter,
            Key::Space => return TinKey::Space,
            Key::NumLock => return TinKey::NumLock,
            Key::Apostrophe => return TinKey::Apostrophe,
            Key::Backslash => return TinKey::Backslash,
            Key::Comma => return TinKey::Comma,
            Key::Equal => return TinKey::Equals,
            Key::LeftAlt => return TinKey::LAlt,
            Key::LeftBracket => return TinKey::LBracket,
            Key::LeftControl => return TinKey::LControl,
            Key::LeftShift => return TinKey::LShift,
            Key::Minus => return TinKey::Minus,
            Key::Period => return TinKey::Period,
            Key::RightAlt => return TinKey::RAlt,
            Key::RightBracket => return TinKey::RBracket,
            Key::RightControl => return TinKey::RControl,
            Key::RightShift => return TinKey::RShift,
            Key::Semicolon => return TinKey::Semicolon,
            Key::Slash => return TinKey::Slash,
            Key::Tab => return TinKey::Tab,
            Key::GraveAccent => return TinKey::GraveAccent,
            Key::CapsLock => return TinKey::CapsLock,
            Key::KpDecimal => return TinKey::NumpadDecimal,
            Key::KpDivide => return TinKey::NumpadDivide,
            Key::KpMultiply => return TinKey::NumpadMultiply,
            Key::KpSubtract => return TinKey::NumpadSubtract,
            Key::KpAdd => return TinKey::NumpadAdd,
            Key::KpEnter => return TinKey::NumpadEnter,
            Key::KpEqual => return TinKey::NumpadEquals,
            Key::Unknown => return TinKey::Unknown,
            _ => TinKey::Unknown,
        }
    }
}
