use luminance_glfw::{GL33Context, GlfwSurface, GlfwSurfaceError};
use luminance_windowing::{WindowDim, WindowOpt};

use glfw::{Action, Context as _, Key, WindowEvent};
use luminance::{
    context::GraphicsContext as _,
    pipeline::{PipelineError, PipelineState, Render},
    render_state::RenderState,
    shader::Program,
    tess::{Mode, Tess},
};

use std::{array, collections::VecDeque, fs::copy, io::{self, Write}, time::{Duration, Instant}, usize};

use luminance_gl::gl33::GL33 as GLBackend;

const VS_STR: &str = include_str!("Tin/shaders/vertexshader.glsl");
const FS_STR: &str = include_str!("Tin/shaders/fragmentshader.glsl");


use crate::Tin::{Float, TController::{TController, TinController}, TinRenderer, get_tin};

use super::Tin::{
    TShapes::TinShape as Shape,
    TVertex::{TinVertex as Vertex, TinVertexSemantics as VertexSemantics}
};

use std::collections::VecDeque as Queue;

pub struct App {
    controller: Box<dyn TController>
}

impl Default for App {
    fn default() -> Self {
        let app = App::new();
        println!("Shape pushed");
        return app;
    }
}

impl App {
    pub fn new() -> App {
        let shapes = Queue::<Shape>::new();
        App {
            controller: Box::new(TinController::default())
        }
    }

    pub fn new_with_controller(controller: Box<dyn TController>) -> App {
        let shapes = Queue::<Shape>::new();
        App {
            controller: controller
        }
    }

    fn prepareShapesForRender(context: &mut GL33Context) -> Queue<Tess<GLBackend, Vertex>> {
        println!("TinApp::prepareShapesForRender()");
        let mut tesses: Queue<Tess<GLBackend, Vertex>> = Queue::new();
        let shapes = &mut super::Tin::get_tin().render.shape_queue;
        while !shapes.is_empty() {
            let shape = shapes.pop_front().unwrap();

            if !shape.shouldDraw() { continue; };
            let vertexCount = shape.getVertices().len();
            let drawMode: Mode;
            match vertexCount {
                1 => panic!("Shape had one vertex"),
                2 => panic!("Shape had two vertices"), // drawMode = Mode::Line,
                3 => drawMode = Mode::Triangle,
                5 => drawMode = Mode::TriangleStrip,
                _ => drawMode = Mode::Patch(vertexCount),
            }
            let vertvec = shape.getVertices();
            
            let shape_tess = context
                .new_tess()
                .set_vertices(vertvec.clone())
                .set_mode(drawMode)
                .build()
                .expect("Could not build tesselation.");
            tesses.push_back(shape_tess);
        }
        tesses
    }

    

    pub fn run(&mut self) -> Result<(), ()> {
        eprintln!("TinApp::run()");
        // Application logic here
        let start_time = Instant::now();


        let mut surface = produce_graphics_surface(&self.controller);

        let ctxt = &mut surface.context;
        let events = &surface.events_rx;

        // This being mutable lets us resize it.
        let mut back_buffer = ctxt.back_buffer().expect("Could not get back buffer");

        // Includes the VertexSemantics generic type so the shader program can be checked against the Tess it's used with,
        // so vertex definitions are consistent
        let mut program = ctxt
            .new_shader_program::<VertexSemantics, (), ()>()
            .from_strings(VS_STR, None, None, FS_STR)
            .unwrap()
            .ignore_warnings();

        let mut last_frame_time = Instant::now();

        'apploop: loop {

            {
                self.controller.get_view_mut().draw();
            }
            

            // handle events
            ctxt.window.glfw.poll_events(); // Fill receiver with events
            for (_, event) in glfw::flush_messages(&events) {
                match event {
                    // End loop
                    WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                        break 'apploop
                    }
                    WindowEvent::Char(c) => { self.controller.keyDown(c) },
                    WindowEvent::CharModifiers(c, m) => { todo!() },
                    WindowEvent::FramebufferSize(..) => back_buffer = ctxt.back_buffer().unwrap(),
                    WindowEvent::Key(k, _, _, _) => {},
                    WindowEvent::Pos(_, _) => {}
                    WindowEvent::Size(_, _) => {}
                    WindowEvent::Refresh => {println!("Window refresh event")}
                    WindowEvent::Focus(_) => {}
                    WindowEvent::Iconify(_) => {}
                    WindowEvent::MouseButton(_, _, _) => {}
                    WindowEvent::CursorPos(_, _) => {}
                    WindowEvent::CursorEnter(_) => {}
                    WindowEvent::Scroll(_, _) => {}
                    WindowEvent::FileDrop(_) => {}
                    WindowEvent::Maximize(_) => {}
                    WindowEvent::ContentScale(_, _) => {}
                }
            }


            let mut tesses: Queue<Tess<GLBackend, Vertex>> = App::prepareShapesForRender(ctxt);

            // rendering code goes here

            let bg_color;
            {
                let bg_render_color;
                {
                    bg_render_color = get_tin().render.get_background_color();
                }
                {
                    bg_color = [bg_render_color.red as Float, bg_render_color.green as Float, 
                    bg_render_color.blue as Float, bg_render_color.alpha as Float]
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
                pace_frames(self.controller.get_view().get_fps(), last_frame_time);
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

fn produce_graphics_surface(controller: &Box<dyn TController>) -> GlfwSurface {
    let view_frame = controller.get_view().get_dimensions();

    let view_title = controller.get_view().get_title();

    let win_opt = WindowOpt::default()
        .set_dim(WindowDim::Windowed {
            width: view_frame.get_width(),
            height: view_frame.get_height(),
        })
        .set_cursor_mode(luminance_windowing::CursorMode::Visible);

    GlfwSurface::new_gl33(view_title, win_opt).expect("GlfwSurface could not be built")
}

fn pace_frames(target_fps: u16, last_frame_time: Instant) {
    let mut target_FPS_local: u32 = target_fps as u32;
    
    if target_FPS_local == 0 {target_FPS_local = 1};
    let frametime_ms: u32 = 1000 / target_FPS_local;
    let safe_sleep_period: Duration =
        Duration::from_millis((frametime_ms as f32 * (9f32 / 10f32)) as u64);
    if Instant::now().duration_since(last_frame_time).as_millis()
        < safe_sleep_period.as_millis()
    {
        //println!("Met threshold for sleep, duration is {}",safe_sleep_period.as_millis().to_string() );
        std::thread::sleep(safe_sleep_period);
    }

    while (Instant::now().duration_since(last_frame_time).as_millis() as u32) < frametime_ms {
        //println!("Spinlocking, duration is {}",Instant::now().duration_since(last_frame_time).as_millis().to_string() );
    }
}