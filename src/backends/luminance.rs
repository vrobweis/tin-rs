//
//  CoreGraphicsRenderer.swift
//  Tin
//
//  Created by Loren Olson on 1/4/17.
//  Created at the School of Arts, Media and Engineering,
//  Herberger Institute for Design and the Arts,
//  Arizona State University.
//  Copyright (c) 2017 Arizona Board of Regents on behalf of Arizona State University
//

pub(crate) mod arc;
pub(crate) mod ellipse;
pub(crate) mod image;
pub(crate) mod line;
pub(crate) mod path;
pub(crate) mod rect;
pub(crate) mod state;
pub(crate) mod text;
pub(crate) mod triangle;

use luminance_gl::gl33::GL33;

use glfw::{Action, Context, WindowEvent};
use luminance::{
    context::GraphicsContext as _,
    pipeline::{PipelineError, PipelineState, Render},
    render_state::RenderState,
    tess::{Mode, Tess},
};
use luminance_glfw::{GL33Context, GlfwSurface};
use luminance_windowing::{WindowDim, WindowOpt};

use super::{
    super::{Float, 
        color::{TColor}, 
        controller::TinController, vertex::TinVertexSemantics,
        context::{
            get_tin
        },
        point::TinPoint, 
        shapes::{TinShape}, vector2::TVector2, vertex::TinVertex,
        context::get_tin_mut, event::TinEvent, scene::TScene
    },
    TinRenderer, TBackend
};

use std::{collections::VecDeque as Queue, time::{Instant}};




fn make_vertex(point: &TinPoint, color: &impl TColor) -> TinVertex {
    TinVertex::new_from_point_and_color(point, color)
}

fn make_shape_from_vertex_vec(vertices: Vec<TinVertex>) -> TinShape {
    TinShape::new(vertices)
}

fn make_shape_from_vector_vec(points: Vec<TVector2>, color: &impl TColor) -> TinShape {
    let mut vertices = Vec::<TinVertex>::new();
    for point in points {
        let vertex = TinVertex::new_from_vector_and_color(&point, color);
        vertices.push(vertex);
    }
    return make_shape_from_vertex_vec(vertices);
}

pub struct LuminanceBackend {
    pub shape_queue: Queue<TinShape>,

    save_state: bool,
    saved_shape_queue: Queue<TinShape>,

    // pub delegate: TinContext,
    
    pub use_layer: bool,

    path_started: bool,
    path_vertices: Queue<TinVertex>
}

impl LuminanceBackend {
    
    fn enqueue_shape(&mut self, mut points: Vec<TVector2>, brush: TBrush, state: DrawState) {
        eprintln!("LuminanceBackend::enqueue_shape()");

        for p in &mut points {
            p.rotate(state.rotation)
        }
        
        let shape_queue = &mut self.shape_queue;

        match brush {
            TBrush::Fill(c) => {
                shape_queue.push_back(make_shape_from_vector_vec(points, &c))
            },
            TBrush::Stroke(c) => shape_queue.push_back(make_shape_from_vector_vec(points, &c)),
            TBrush::FillAndStroke(f, s) => {
                shape_queue.push_back(make_shape_from_vector_vec(points.clone(), &f));
                shape_queue.push_back(make_shape_from_vector_vec(points, &s))
            },
            TBrush::Disabled => todo!(),
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
        self.saved_shape_queue.clear();

        self.save_state = false;

        
        
        
        self.use_layer = false;

        self.path_started = false;
        self.path_vertices.clear();

        assert_eq!(self.shape_queue.len(), 0);
        assert_eq!(self.saved_shape_queue.len(), 0);
        assert_eq!(self.save_state, false);
        
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

            save_state: false,
            saved_shape_queue: Queue::<TinShape>::new(),

            //delegate: TinContext::init(),// Probably need to change this when the context is fully implemented
            
            use_layer: false,

            path_started: false,
            path_vertices: Queue::<TinVertex>::new()
        }
    }

    fn run<S, H>(mut controller: TinController<S, H>) -> Result<(), ()> where S: TScene, H: Fn(TinEvent, &mut S, &mut TinView) {
        eprintln!("LuminanceBackend::run()");
        // Application logic here

        let mut view = controller.tin_view;

        let mut surface = produce_graphics_surface(&view);


        let ctxt = &mut surface.context;
        let events = &surface.events_rx;
    
        // This being mutable lets us resize it.
        let mut back_buffer = ctxt.back_buffer().expect("Could not get back buffer");
    
    
        const VS_STR: &str = include_str!("shaders/vertexshader.glsl");
        const FS_STR: &str = include_str!("shaders/fragmentshader.glsl");
        // Includes the TinVertexSemantics generic type so the shader program can be checked against the Tess it's used with,
        // so vertex definitions are consistent
        let mut program = ctxt
            .new_shader_program::<TinVertexSemantics, (), ()>()
            .from_strings(VS_STR, None, None, FS_STR)
            .unwrap()
            .ignore_warnings();
    
        let mut last_frame_time = Instant::now();

        let mut mouse_pos: TinPoint = TinPoint::default();

        

        let mut scene = controller.scene;

        let handler = &mut controller.handler;
        

        'apploop: loop {

            TinController::<S,H>::draw(&mut scene);
            

    
            // handle events
            ctxt.window.glfw.poll_events(); // Fill receiver with events
            for (_, event) in glfw::flush_messages(&events) {
                match event {
                    // End loop
                    WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                        break 'apploop
                    }
                    WindowEvent::Char(_) => {  },
                    WindowEvent::CharModifiers(_, _) => {  },
                    WindowEvent::FramebufferSize(..) => back_buffer = ctxt.back_buffer().unwrap(),

                    WindowEvent::Key(k, _, Action::Press, _) => {
                        TinController::on_event(TinEvent::KeyDown(map_key_to_tin_key(k).unwrap()), &mut scene, &mut view, handler)
                    },
                    WindowEvent::Key(k, _, Action::Release, _) => {

                        TinController::on_event(TinEvent::KeyUp(map_key_to_tin_key(k).unwrap()), &mut scene, &mut view, handler)
                    },
                    WindowEvent::Key(_k, _, _, _) => {  },

                    WindowEvent::Pos(_, _) => {}
                    WindowEvent::Size(_, _) => {}
                    WindowEvent::Refresh => {println!("Window refresh event");}
                    WindowEvent::Focus(_) => {}
                    WindowEvent::Iconify(_) => {}

                    WindowEvent::MouseButton(glfw::MouseButton::Button1, Action::Press, _) => {
                        TinController::on_event(TinEvent::MouseDown(mouse_pos.clone()), &mut scene, &mut view, handler)
                    }
                    WindowEvent::MouseButton(glfw::MouseButton::Button1, Action::Release, _) => {
                        TinController::on_event(TinEvent::MouseUp(mouse_pos.clone()), &mut scene, &mut view, handler)
                    }
                    WindowEvent::MouseButton(glfw::MouseButton::Button1, Action::Repeat, _) => {
                        
                        TinController::on_event(TinEvent::MouseDown(mouse_pos.clone()), &mut scene, &mut view, handler)
                    }

                    WindowEvent::MouseButton(glfw::MouseButton::Button2, Action::Press, _) => {

                        TinController::on_event(TinEvent::RightMouseDown(mouse_pos.clone()), &mut scene, &mut view, handler)
                    }
                    WindowEvent::MouseButton(glfw::MouseButton::Button2, Action::Release, _) => {
                        TinController::on_event(TinEvent::RightMouseUp(mouse_pos.clone()), &mut scene, &mut view, handler)
                    }
                    WindowEvent::MouseButton(glfw::MouseButton::Button2, Action::Repeat, _) => {
                        
                        TinController::on_event(TinEvent::RightMouseDown(mouse_pos.clone()), &mut scene, &mut view, handler)
                    }

                    WindowEvent::MouseButton(_, Action::Press, _) => {
                        TinController::on_event(TinEvent::OtherMouseDown(mouse_pos.clone()), &mut scene, &mut view, handler)
                    }
                    WindowEvent::MouseButton(_, Action::Release, _) => {
                        TinController::on_event(TinEvent::OtherMouseUp(mouse_pos.clone()), &mut scene, &mut view, handler)
                    }
                    WindowEvent::MouseButton(_, Action::Repeat, _) => {
                        TinController::on_event(TinEvent::OtherMouseDown(mouse_pos.clone()), &mut scene, &mut view, handler)
                    }
                    
                    WindowEvent::CursorPos(x, y) => {
                        mouse_pos = TinPoint::new_from_coords(x,y);
                        get_tin_mut().mouse_moved(mouse_pos.clone());
                    }
                    WindowEvent::CursorEnter(_) => {}
                    WindowEvent::Scroll(_, _) => {}
                    WindowEvent::FileDrop(_) => {}
                    WindowEvent::Maximize(_) => {}
                    WindowEvent::ContentScale(_, _) => {}
                }
            }
            
    
            let mut tesses: Queue<Tess<luminance_gl::gl33::GL33, TinVertex>> = prepare_shapes_for_render(ctxt);
    
            // rendering code goes here
    
            let bg_color;
            {
                let bg_render_color;
                {
                    bg_render_color = get_tin().get_background_color();
                }
                {
                    bg_color = [bg_render_color.get_red() as Float, bg_render_color.get_green() as Float, 
                    bg_render_color.get_blue() as Float, bg_render_color.get_alpha() as Float]
                }
            }
            
    
            let render: Render<PipelineError> = ctxt
                .new_pipeline_gate()
                .pipeline(
                    // the frame buffer to write to
                    &back_buffer,
                    // object containing color to use when clearing framebuffer color buffers
                    &PipelineState::default().set_clear_color(bg_color),
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
                super::pace_frames(view.get_fps(), last_frame_time);
                ctxt.window.swap_buffers();
                last_frame_time = Instant::now();
            } else {
                break 'apploop;
            }
        }
        println!(
            "Successfully executed Tin. {}",
            "Now, to build drawing code!"
        );
        Ok(())
    }
}



fn produce_graphics_surface(view_ref: &TinView) -> GlfwSurface {
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
    println!("luminance::prepare_shapes_for_render()");
    let mut tesses: Queue<Tess<GL33, TinVertex>> = Queue::new();
    let shapes = &mut get_tin_mut().render.shape_queue;
    while !shapes.is_empty() {
        let shape = shapes.pop_front().unwrap();

        if !shape.should_draw() { continue; };
        let vertex_count = shape.get_vertices().len();
        let draw_mode: Mode;
        match vertex_count {
            1 => panic!("Shape had one vertex"),
            2 => panic!("Shape had two vertices"), // draw_mode = Mode::Line,
            3 => draw_mode = Mode::Triangle,
            5 => draw_mode = Mode::TriangleStrip,
            _ => draw_mode = Mode::Patch(vertex_count),
        }
        let vertvec = shape.get_vertices();
        
        let shape_tess = context
            .new_tess()
            .set_vertices(vertvec.clone())
            .set_mode(draw_mode)
            .build()
            .expect("Could not build tesselation.");
        tesses.push_back(shape_tess);
    }
    tesses
}

use glfw::Key;
use crate::{context::{DrawState, TBrush}, key::TinKey, view::TinView};

fn map_key_to_tin_key(k: glfw::Key) -> Option<TinKey> {

    match k {
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
        Key::Num1 => {return Some(TinKey::One)}
        Key::Num2 => {return Some(TinKey::Two)}
        Key::Num3 => {return Some(TinKey::Three)}
        Key::Num4 => {return Some(TinKey::Four)}
        Key::Num5 => {return Some(TinKey::Five)}
        Key::Num6 => {return Some(TinKey::Six)}
        Key::Num7 => {return Some(TinKey::Seven)}
        Key::Num8 => {return Some(TinKey::Eight)}
        Key::Num9 => {return Some(TinKey::Nine)}
        Key::Num0 => {return Some(TinKey::Zero)}
        Key::Kp0 => {return Some(TinKey::Zero)}
        Key::Kp1 => {return Some(TinKey::One)}
        Key::Kp2 => {return Some(TinKey::Two)}
        Key::Kp3 => {return Some(TinKey::Three)}
        Key::Kp4 => {return Some(TinKey::Four)}
        Key::Kp5 => {return Some(TinKey::Five)}
        Key::Kp6 => {return Some(TinKey::Six)}
        Key::Kp7 => {return Some(TinKey::Seven)}
        Key::Kp8 => {return Some(TinKey::Eight)}
        Key::Kp9 => {return Some(TinKey::Nine)}
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
        Key::PrintScreen => {return Some(TinKey::Snapshot)}
        Key::ScrollLock => {return Some(TinKey::Scroll)}
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
        Key::Backspace => {return Some(TinKey::Backspace)}
        Key::Enter => {return Some(TinKey::Enter)}
        Key::Space => {return Some(TinKey::Space)}
        Key::NumLock => {return Some(TinKey::NumLock)}
        Key::Apostrophe => {return Some(TinKey::Apostrophe)}
        Key::Backslash => {return Some(TinKey::Backslash)}
        Key::Comma => {return Some(TinKey::Comma)}
        Key::Equal => {return Some(TinKey::Equals)}
        Key::LeftAlt => {return Some(TinKey::LAlt)}
        Key::LeftBracket => {return Some(TinKey::LBracket)}
        Key::LeftControl => {return Some(TinKey::LControl)}
        Key::LeftShift => {return Some(TinKey::LShift)}
        Key::Minus => {return Some(TinKey::Minus)}
        Key::Period => {return Some(TinKey::Period)}
        Key::RightAlt => {return Some(TinKey::RAlt)}
        Key::RightBracket => {return Some(TinKey::RBracket)}
        Key::RightControl => {return Some(TinKey::RControl)}
        Key::RightShift => {return Some(TinKey::RShift)}
        Key::Semicolon => {return Some(TinKey::Semicolon)}
        Key::Slash => {return Some(TinKey::Slash)}
        Key::Tab => {return Some(TinKey::Tab)}
        Key::GraveAccent => {return Some(TinKey::GraveAccent)}
        Key::CapsLock => {return Some(TinKey::CapsLock)}
        Key::KpDecimal => {return Some(TinKey::NumpadDecimal)}
        Key::KpDivide => {return Some(TinKey::NumpadDivide)}
        Key::KpMultiply => {return Some(TinKey::NumpadMultiply)}
        Key::KpSubtract => {return Some(TinKey::NumpadSubtract)}
        Key::KpAdd => {return Some(TinKey::NumpadAdd)}
        Key::KpEnter => {return Some(TinKey::NumpadEnter)}
        Key::KpEqual => {return Some(TinKey::NumpadEquals)}
        Key::Unknown => {return Some(TinKey::Unknown)}
        _ => None
    }
}