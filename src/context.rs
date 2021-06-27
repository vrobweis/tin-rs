
use crate::{CurrentBackend, Double, ULong, backends::TBackend, brush::TBrush, color::{
        DEFAULT_COLOR_FILL, DEFAULT_COLOR_STROKE, DEFAULT_COLOR_BACKGROUND,
        TColor, TinColor
    }, draw::DrawCall, frame::TinFrame, point::TPoint, point::TinPoint};

use lazy_static;
use std::{collections::VecDeque, sync::{
    RwLock,
    RwLockWriteGuard,
    RwLockReadGuard
}};

lazy_static::lazy_static! {
    static ref TIN_LOCK: RwLock<TinContext<CurrentBackend>> = RwLock::new(TinContext::new());
}

pub(crate) fn get_tin<'t>() -> RwLockReadGuard<'t, TinContext<CurrentBackend>> {
    eprintln!("get_tin()");
    return TIN_LOCK.read().unwrap();
}

pub(crate) fn get_tin_mut<'t>() -> RwLockWriteGuard<'t, TinContext<CurrentBackend>> {
    eprintln!("get_tin_mut()");
    return TIN_LOCK.write().unwrap();
}



#[derive(Debug, Clone, Copy)]
pub(crate) struct DrawState {
    pub(crate) rotation: Double,
    pub(crate) scale: Double,
    pub(crate) translation: (Double, Double),
}

impl Default for DrawState {
    fn default() -> Self {
        Self {
            rotation: 0.0,
            scale: 1.0,
            translation: (0.0,0.0)
        }
    }
}

type DrawQueue = VecDeque<crate::draw::DrawCall>;

pub(crate) struct TinContext<T: TBackend> {
    pub size: [Double;2],
    pub width: Double,
    pub height: Double,
    pub mid_x: Double,
    pub mid_y: Double,
    pub mouse_pos: TinPoint,
    pub prev_mouse_pos: TinPoint,
    pub mouse_pressed: bool,
    frame_count: ULong,

    pub state: DrawState,

    pub(crate) calls: DrawQueue,

    pub fill: bool,
    pub stroke: bool,
    pub(crate) current_fill_color: TinColor,
    pub(crate) current_stroke_color: TinColor,
    pub(crate) current_background_color: TinColor,

    pub(crate) line_width: Double,

    pub(crate) path_vertex_count: ULong,

    pub(crate) render: Box<T>
}

impl<T: TBackend> TinContext<T> {

    /// TODO: Document this method.
    pub fn new() -> Self {
        Self {
            fill: true,
            stroke: true,
            size: [0.,0.],
            width: 0.0,
            height: 0.0,
            mid_x: 0.0,
            mid_y: 0.0,
            mouse_pos: TinPoint::default(),
            prev_mouse_pos: TinPoint::default(),
            mouse_pressed: false,
            frame_count: 0,

            state: DrawState {
                rotation: 0.0,
                scale: 1.0,
                translation: (0.0,0.0),
            },

            calls: VecDeque::new(),

            current_fill_color: DEFAULT_COLOR_FILL,
            current_stroke_color: DEFAULT_COLOR_STROKE,
            current_background_color: DEFAULT_COLOR_BACKGROUND,

            line_width: 0.05,

            path_vertex_count: 0,

            render: Box::new(T::new())
        }
    }

    pub fn get_brush(&self) -> TBrush {
        if self.fill & self.stroke {
            TBrush::FillAndStroke(self.current_fill_color, self.current_stroke_color)
        } else if self.fill & !self.stroke {
            TBrush::Fill(self.current_fill_color)
        } else if !self.fill & self.stroke {
            TBrush::Stroke(self.current_stroke_color)
        } else {
            TBrush::Disabled
        }
    }

    /// TODO: Document this method.
    pub fn prepare(&mut self, frame: TinFrame) {
        // self.render.delegate = self;
        self.render.prepare(frame);
        self.reset(frame.get_width() as Double, frame.get_height() as Double);
    }



    // MARK: - Rendering cycle

    /// TODO: Document this method.
    pub fn prepare_for_update(&mut self) {
        self.reset(self.width, self.height);
        self.update_frame_count();
        {
            assert_eq!(self.current_fill_color, DEFAULT_COLOR_FILL);
            assert_eq!(self.current_stroke_color, DEFAULT_COLOR_STROKE);
            assert_eq!(self.current_background_color, DEFAULT_COLOR_BACKGROUND);
        }
        self.render.prepare_for_update();
    }

    /// TODO: Document this method.
    pub fn did_finish_update(&mut self) {
        self.render.did_finish_update();
    }

    /// TODO: Document this method.
    pub fn reset_size(&mut self, width: Double, height: Double) {
        self.size = [width, height];
        self.width = width;
        self.height = height;
        self.mid_x = width / 2.0;
        self.mid_y = height / 2.0;
    }


    /**
      Reset variables needed to draw next frame:
            - buffer size should match the window size
            - fill and stroke colors
            - line width
    */
    pub fn reset(&mut self, width: Double, height: Double) {
        self.reset_size(width, height);
        self.fill = true;
        self.stroke = true;
        self.line_width = 0.05;

        self.set_fill_color(DEFAULT_COLOR_FILL);
        self.set_stroke_color(DEFAULT_COLOR_STROKE);
        self.set_background_color(DEFAULT_COLOR_BACKGROUND);
    }


    /// TODO: Document this function.
    pub fn mouse_moved(&mut self, to_point: impl TPoint) {
        self.prev_mouse_pos = self.mouse_pos.clone();
        {
            self.mouse_pos.set_x(to_point.get_x());
            self.mouse_pos.set_y(to_point.get_y());
        }
    }

    pub fn get_frame_count(&self) -> ULong {
        self.frame_count
    }

    /// TODO: Document this function.
    fn update_frame_count(&mut self) {
        self.frame_count += 1;
    }

    // MARK: - Color state

    pub fn set_stroke_color(&mut self, color: impl TColor) {
        self.current_stroke_color = TinColor::from(color);
    }


    pub fn set_fill_color(&mut self, color: impl TColor) {
        self.current_fill_color = TinColor::from(color);
    }


    pub fn set_background_color(&mut self, color: impl TColor) {
        self.current_background_color = TinColor::from(color);
    }


    pub fn get_stroke_color(&self) -> impl TColor {
        return self.current_stroke_color
    }


    pub fn get_fill_color(&self) -> impl TColor {
        return self.current_fill_color
    }

    pub fn get_background_color(&self) -> impl TColor {
        return self.current_background_color
    }


    pub fn process_draw_calls(&mut self) {
        let render = &mut self.render;

        let calls: &mut DrawQueue = &mut self.calls;

        let mut fill_color = DEFAULT_COLOR_FILL;
        let mut stroke_color = DEFAULT_COLOR_STROKE;
        let mut should_fill = true;let mut should_stroke = true;
        let mut line_width = self.line_width;
        let mut state = DrawState::default();
        let mut pushed_state = None;
        //let mut path_vertices = Vec::<crate::vertex::TinVertex>::new();
        while calls.len() > 0 {
            let call = calls.pop_front().expect("Draw call list should have had a value.");

            let brush = if should_fill && should_stroke {
                TBrush::FillAndStroke(fill_color, stroke_color)
            } else if should_fill && !should_stroke {
                TBrush::Fill(fill_color)
            } else if !should_fill && should_stroke {
                TBrush::Stroke(stroke_color)
            } else {
                TBrush::Disabled
            };
            match call {
                DrawCall::Background(color) => self.current_background_color = color,
                DrawCall::Fill(color) => fill_color = color,
                DrawCall::Stroke(color) => stroke_color = color,
                DrawCall::SetAlpha(alpha) => {
                    fill_color.set_alpha(alpha);
                    stroke_color.set_alpha(alpha);
                },

                DrawCall::PushState => pushed_state = Some(state),
                DrawCall::PopState => {
                    match pushed_state {
                        Some(s) => {state = s; pushed_state = None},
                        None => {eprintln!("WARNING: PopState was invoked without state being pushed.")}
                    }
                },
                DrawCall::Translate(dx, dy) => {
                    state.translation.0 += dx.clone();
                    state.translation.1 += dy.clone();
                },
                DrawCall::Rotate(theta) => state.rotation += theta.clone(),
                DrawCall::Scale(amount) => state.scale += amount.clone(),
                DrawCall::LineWidth(width) => line_width = width,

                DrawCall::Arc(arc) => render.arc(arc, brush, state),
                DrawCall::Ellipse(rect) => render.ellipse_in_tinrect(&rect, brush, state),
                DrawCall::Line(point1, point2) => render.line(point1, point2, line_width, brush, state),
                DrawCall::Rect(rect) => render.rect_with_tinrect(&rect, brush, state),
                DrawCall::RoundedRect(rounded_rect) => render.rounded_rect(&rounded_rect, brush, state),
                DrawCall::Triangle(triangle) => render.triangle(triangle, brush, state),
                DrawCall::PathBegin => todo!(),
                DrawCall::PathVertex(point) => {render.path_vertex(&point); self.path_vertex_count += 1},
                DrawCall::PathAddCurve(_wrapper) => {/* render.path_add_curve(); */self.path_vertex_count += 4;todo!()},
                DrawCall::PathEnd => {self.path_vertex_count = 0; /* path_vertices.clear()*/ },
                DrawCall::FillEnable => should_fill = true,
                DrawCall::FillDisable => should_fill = false,
                DrawCall::StrokeEnable => should_stroke = true,
                DrawCall::StrokeDisable => should_stroke = false,
                #[cfg(image)]
                DrawCall::Image(wrapper) => render.image_with_size_and_resize(wrapper.image, wrapper.center, wrapper.width, wrapper.height, wrapper.resize, state),
                #[cfg(text)]
                DrawCall::Text(_) => todo!(),

                call => eprintln!("{:?}",call)
            }
        }
    }


}



