//
//  Tin.swift
//  Tin
//
//  Created by Loren Olson on 12/28/16.
//  Created at the School of Arts, Media and Engineering,
//  Herberger Institute for Design and the Arts,
//  Arizona State University.
//  Copyright (c) 2017 Arizona Board of Regents on behalf of Arizona State University
//
use std::sync::{
    RwLock,
    RwLockWriteGuard,
    RwLockReadGuard
};

use crate::{color::{self, TColor}, controller::TinController, event::TinEvent, scene::TScene, view::TinView};

use super::{
    frame::TinFrame, 
    point::TinPoint,
    backends::TBackend,
    backends::luminance::LuminanceBackend,
    //backends::nannou::NannouBackend,
    color::{DEFAULT_COLOR_FILL, DEFAULT_COLOR_STROKE, DEFAULT_COLOR_BACKGROUND, TinColor},
    Double,
};

extern crate lazy_static;

type BACKEND = LuminanceBackend;

lazy_static::lazy_static! {
    static ref TIN_LOCK: RwLock<TinContext<BACKEND>> = RwLock::new(TinContext::new());
}

pub(crate) fn get_tin<'t>() -> RwLockReadGuard<'t, TinContext<BACKEND>> {
    eprintln!("get_tin()");
    return TIN_LOCK.read().unwrap();
}

pub(crate) fn get_tin_mut<'t>() -> RwLockWriteGuard<'t, TinContext<BACKEND>> {
    eprintln!("get_tin_mut()");
    return TIN_LOCK.write().unwrap();
}


pub fn run<S,H>(tin_view: TinView, scene: S, handler: H) -> Result<(), ()> where S: TScene, H: Fn(TinEvent, &mut S, &mut TinView)  {
    BACKEND::run(TinController::new(tin_view, scene, handler))
}

pub(crate) enum DrawType {
    Fill(TinColor), Stroke(TinColor), FillAndStroke(TinColor, TinColor), Disabled
}

pub(crate) struct DrawState {
    pub(crate) rotation: Double,
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
    pub frame_count: u64,

    pub(crate) rotation: Double,
    pub(crate) scale: Double,
    pub(crate) translation: (Double, Double),

    pub fill: bool,
    pub stroke: bool,
    pub(crate) current_fill_color: TinColor,
    pub(crate) current_stroke_color: TinColor,
    pub(crate) current_background_color: TinColor,

    pub(crate) line_width: Double,

    pub(crate) path_vertex_count: u64,
    
    pub(crate) render: Box<T>,
    //pub(crate) controller_constructor: FnOnce() -> impl TController
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
            
            rotation: 0.0,
            scale: 1.0,
            translation: (0.0,0.0),

            current_fill_color: DEFAULT_COLOR_FILL,
            current_stroke_color: DEFAULT_COLOR_STROKE,
            current_background_color: DEFAULT_COLOR_BACKGROUND,

            line_width: 0.05,

            path_vertex_count: 0,
            
            render: Box::new(T::new())
        }
    }

    pub fn get_brush(&self) -> DrawType {
        if self.fill & self.stroke {
            DrawType::FillAndStroke(self.current_fill_color, self.current_stroke_color)
        } else if self.fill & !self.stroke {
            DrawType::Fill(self.current_fill_color)
        } else if !self.fill & self.stroke {
            DrawType::Stroke(self.current_stroke_color)
        } else {
            DrawType::Disabled
        }
    }

    pub fn get_state(&self) -> DrawState {
        DrawState {
            rotation: self.rotation
        }
    }

    /// TODO: Document this method.
    pub fn prepare(&mut self, frame: TinFrame) {
        println!("context::prepare()");
        // self.render.delegate = self;
        self.render.prepare(frame);
        self.reset(frame.get_width() as Double, frame.get_height() as Double);
    }

    
    
    // MARK: - Rendering cycle
    
    /// TODO: Document this method.
    pub fn prepare_for_update(&mut self) {
        println!("context::prepare_for_update()");
        self.set_fill_color_with_color(&color::DEFAULT_COLOR_FILL);
        self.set_stroke_color_with_color(&color::DEFAULT_COLOR_STROKE);
        self.current_background_color = color::DEFAULT_COLOR_BACKGROUND;

        {
            assert_eq!(self.current_fill_color, color::DEFAULT_COLOR_FILL);
            assert_eq!(self.current_stroke_color, color::DEFAULT_COLOR_STROKE);
            assert_eq!(self.current_background_color, color::DEFAULT_COLOR_BACKGROUND);
        }
        self.render.prepare_for_update();
    }
    
    /// TODO: Document this method.
    pub fn did_finish_update(&mut self) {
        println!("context::did_finish_update()");
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
    }
    
    
/* 

    // enable "Restore from previous"
    // When enabled, this feature will cause the rendering to be saved in an image buffer.
    // After the double buffer swap, before the next render happens, the image saved
    // in the buffer is restored to the current frame buffer. This allows continuous draw effects.
    // Note, there is a time penalty for saving and restoring the image.
    pub fn enableRestoreFromPrevious(&mut self, ) {
        self.render.use_layer = true
    }
    
    
    pub fn disableRestoreFromPrevious(&mut self, ) {
        self.render.use_layer = false
    }
    */
    

    /// TODO: Document this function.
    pub fn mouse_moved(&mut self, to_point: TinPoint) {
        self.pmouse_pos = self.mouse_pos.clone();
        self.mouse_pos = to_point;
    }
    
    /// TODO: Document this function.
    pub fn update_frame_count(&mut self) {
        self.frame_count += 1;
    }
    
    // MARK: - Color state
    
    pub fn set_stroke_color(&mut self, red: &Double, green: &Double, blue: &Double, alpha: &Double) {
        eprintln!("context::set_stroke_color()");
        //cg.set_stroke_color(red: r, green: g, blue: b, alpha: a);
        self.current_stroke_color = TinColor::new_from_rgba(*red, *green, *blue, *alpha);
    }
    pub fn set_stroke_color_with_color(&mut self, color: &impl TColor) {
        eprintln!("context::set_stroke_color_with_color()");
        self.set_stroke_color(&color.get_red(), &color.get_green(), &color.get_blue(), &color.get_alpha())
    }
    
    
    
    
    pub fn set_fill_color(&mut self, red: &Double, green: &Double, blue: &Double, alpha: &Double) {
        eprintln!("context::set_fill_color()");
        self.current_fill_color = TinColor::new_from_rgba(*red, *green, *blue, *alpha);
    }

    pub fn set_fill_color_with_color(&mut self, color: &impl TColor) {
        eprintln!("context::set_fill_color_with_color()");
        self.set_fill_color(&color.get_red(), &color.get_green(), &color.get_blue(), &color.get_alpha())
    }


    pub fn set_background_color(&mut self, red: &Double, green: &Double, blue: &Double, alpha: &Double) {
        eprintln!("context::set_background_color()");
        self.current_background_color = TinColor::new_from_rgba(*red, *green, *blue, *alpha);
    }

    pub fn set_background_color_with_color(&mut self, color: &impl TColor) {
        eprintln!("context::set_background_color_with_color()");
        self.set_background_color(&color.get_red(), &color.get_green(), &color.get_blue(), &color.get_alpha())
    }
    
    
    pub fn get_stroke_color(&self) -> impl TColor {
        eprintln!("context::get_stroke_color()");
        return self.current_stroke_color
    }
    
    
    pub fn get_fill_color(&self) -> impl TColor {
        eprintln!("context::get_fill_color()");
        return self.current_fill_color
    }

    pub fn get_background_color(&self) -> impl TColor {
        return self.current_background_color
    }
    
    pub fn set_alpha(&mut self, alpha: &Double) {
        eprintln!("context::set_alpha()");
        {
            let current_color = self.current_fill_color;
            let red = current_color.red;
            let green = current_color.green;
            let blue = current_color.blue;
            self.current_fill_color = TinColor::new_from_rgba(red, green, blue, *alpha);
        }
        {
            let current_color = self.current_stroke_color;
            let red = current_color.red;
            let green = current_color.green;
            let blue = current_color.blue;
            self.current_stroke_color =TinColor::new_from_rgba(red, green, blue, *alpha);
        }
        
        //cg.setAlpha(CGFloat(alpha));
    }
}


