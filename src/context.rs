
use crate::{
    frame::TinFrame, 
    point::TinPoint,
    backends::TBackend,
    color::{
        DEFAULT_COLOR_FILL, DEFAULT_COLOR_STROKE, DEFAULT_COLOR_BACKGROUND, 
        TColor, TinColor
    },
    Double,
    CurrentBackend,
    ULong, 
    point::TPoint
};

use lazy_static;
use std::sync::{
    RwLock,
    RwLockWriteGuard,
    RwLockReadGuard
};

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

pub(crate) enum TBrush {
    Fill(TinColor), Stroke(TinColor), FillAndStroke(TinColor, TinColor), Disabled
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct DrawState {
    pub(crate) rotation: Double,
    pub(crate) scale: Double,
    pub(crate) translation: (Double, Double),
}

pub(crate) struct TinContext<T: TBackend> {
    pub size: [Double;2],
    pub width: Double,
    pub height: Double,
    pub mid_x: Double,
    pub mid_y: Double,
    pub mouse_pos: TinPoint,
    pub pmouse_pos: TinPoint,
    pub mouse_pressed: bool,
    frame_count: ULong,

    pub state: DrawState,

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
            pmouse_pos: TinPoint::default(),
            mouse_pressed: false,
            frame_count: 0,
            
            state: DrawState {
                rotation: 0.0,
                scale: 1.0,
                translation: (0.0,0.0),
            },
            

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

    pub fn get_state(&self) -> DrawState {
        self.state.clone()
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
        
        self.set_fill_color_with_color(&DEFAULT_COLOR_FILL);
        self.set_stroke_color_with_color(&DEFAULT_COLOR_STROKE);
        self.current_background_color = DEFAULT_COLOR_BACKGROUND;
    }
    

    /// TODO: Document this function.
    pub fn mouse_moved(&mut self, to_point: impl TPoint) {
        self.pmouse_pos = self.mouse_pos.clone();
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
    
    pub fn set_stroke_color(&mut self, red: Double, green: Double, blue: Double, alpha: Double) {
        self.current_stroke_color = TinColor::new_from_rgba(red, green, blue, alpha);
    }
    pub fn set_stroke_color_with_color(&mut self, color: &impl TColor) {
        self.set_stroke_color(color.get_red(), color.get_green(), color.get_blue(), color.get_alpha())
    }
    
    
    
    
    pub fn set_fill_color(&mut self, red: Double, green: Double, blue: Double, alpha: Double) {
        self.current_fill_color = TinColor::new_from_rgba(red, green, blue, alpha);
    }

    pub fn set_fill_color_with_color(&mut self, color: &impl TColor) {
        self.set_fill_color(color.get_red(), color.get_green(), color.get_blue(), color.get_alpha())
    }


    pub fn set_background_color(&mut self, red: Double, green: Double, blue: Double, alpha: Double) {
        self.current_background_color = TinColor::new_from_rgba(red, green, blue, alpha);
    }

    pub fn set_background_color_with_color(&mut self, color: &impl TColor) {
        self.set_background_color(color.get_red(), color.get_green(), color.get_blue(), color.get_alpha())
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
    
    pub fn set_alpha(&mut self, alpha: Double) {
        {
            let current_color = self.current_fill_color;
            let red = current_color.red;
            let green = current_color.green;
            let blue = current_color.blue;
            self.current_fill_color = TinColor::new_from_rgba(red, green, blue, alpha);
        }
        {
            let current_color = self.current_stroke_color;
            let red = current_color.red;
            let green = current_color.green;
            let blue = current_color.blue;
            self.current_stroke_color =TinColor::new_from_rgba(red, green, blue, alpha);
        }
    }
}


