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



// use luminance_glfw::{GL33Context, GlfwSurface, GlfwSurfaceError};
// use luminance_windowing::{WindowDim, WindowOpt};

// use glfw::{Action, Context as _, Key, WindowEvent};
use luminance::{
    context::GraphicsContext as _,
    pipeline::{PipelineError, PipelineState, Render},
    render_state::RenderState,
    shader::Program,
    tess::{Mode, Tess},
};

use super::{Double, DrawType, Float, TColor::{self, DEFAULT_COLOR_BACKGROUND, DEFAULT_COLOR_FILL, DEFAULT_COLOR_STROKE, TinColor}, TFont::TinFont, TFrame::TinFrame, TImage::TinImage, TPoint::TinPoint, TShapes::{TinRect, TinShape}, TVertex::TinVertex, TinContext, TinRenderer, get_tin};

use std::collections::VecDeque as Queue;



#[allow(dead_code)]
fn MakeVertex(point: &TinPoint, color: &TinColor) -> TinVertex {
    let position = super::TVertex::VertexPosition::new([point.x, point.y]);
    TinVertex::new_from_pointAndColor(point, color)
    
}
#[allow(dead_code)]
fn MakeShapeFromVertexVec(vertices: Vec<TinVertex>) -> TinShape {
    TinShape::new(vertices)
}
#[allow(dead_code)]
fn MakeShapeFromPointVec(points: &Vec<TinPoint>, color: &TinColor) -> TinShape {
    let mut vertices = Vec::<TinVertex>::new();
    for point in points {
        let vertex = TinVertex::new_from_pointAndColor(point, color);
        vertices.push(vertex);
    }
    return MakeShapeFromVec(vertices);
}
fn MakeShapeFromVec(vertices: Vec<TinVertex>) -> TinShape {
    TinShape::new(vertices)
}



pub struct LuminanceRenderer {
    pub shape_queue: Queue<TinShape>,

    save_state: bool,
    saved_shape_queue: Queue<TinShape>,

    pub drawType: DrawType,

    // pub delegate: TinContext,
    current_fill_color: TinColor,
    current_stroke_color: TinColor,
    current_background_color: TinColor,
    pub useLayer: bool,

    path_started: bool,
    path_vertices: Queue<TinVertex>
}

impl LuminanceRenderer {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            shape_queue: Queue::<TinShape>::new(),

            save_state: false,
            saved_shape_queue: Queue::<TinShape>::new(),
            drawType: DrawType::Fill,
            //delegate: delegate
            current_fill_color: DEFAULT_COLOR_FILL,
            current_stroke_color: DEFAULT_COLOR_STROKE,
            current_background_color: DEFAULT_COLOR_BACKGROUND,
            useLayer: false,

            path_started: false,
            path_vertices: Queue::<TinVertex>::new()
        }
    }

    fn EnqueueShape(&mut self, points: &Vec<TinPoint>) {
        let draw_type = &self.drawType;
        let fill_color = &self.get_fill_color();
        let stroke_color = &self.get_stroke_color();
        let shape_queue = &mut self.shape_queue;
        
        match draw_type {
            DrawType::Fill => shape_queue.push_back(MakeShapeFromPointVec(points, &fill_color)),
            DrawType::Stroke => shape_queue.push_back(MakeShapeFromPointVec(points, &stroke_color)),
            DrawType::FillAndStroke => { 
                shape_queue.push_back(MakeShapeFromPointVec(points, &fill_color)); 
                shape_queue.push_back(MakeShapeFromPointVec(points, &stroke_color)); 
            }
        }
    }

}

impl TinRenderer for LuminanceRenderer {

    // MARK: - Drawing methods

    fn prepare(&mut self, frame: super::TFrame::TinFrame) {
        todo!("prepare method in LuminanceRenderer not supported yet");
    }

    fn prepareForUpdate(&mut self, frame: TinFrame) {
        
        self.shape_queue.clear();
        assert!(self.shape_queue.len() == 0);

        self.save_state = false;

        self.saved_shape_queue.clear();
        assert!(self.saved_shape_queue.len() == 0);

        self.drawType = DrawType::FillAndStroke;

        self.current_fill_color = TColor::DEFAULT_COLOR_FILL;
        self.current_stroke_color = TColor::DEFAULT_COLOR_STROKE;
        self.current_background_color = TColor::DEFAULT_COLOR_BACKGROUND;

        self.useLayer = false;

        self.path_started = false;
        self.path_vertices.clear();
        assert!(self.path_vertices.len() == 0);

        // call reset method in context that resets variables
        /*  Reset variables needed to draw:
            - buffer size should match the window size
            - fill and stroke colors
            - line width
        */
        
    }

    fn didFinishUpdate(&mut self) {
        eprintln!("LuminanceRenderer::didFinishUpdate")
    }
    
    // Set background
    fn background(&mut self, red: &Double, green: &Double, blue: &Double) {
        self.current_background_color = TinColor::new_from_rgba(*red, *green, *blue, 1.0)
    }

    fn arc(&mut self, x: &Double, y: &Double, radius: &Double, startAngle: &Double, endAngle: &Double) {
        todo!("Arc render for LuminanceRenderer not implemented");
    }
    
    fn ellipse(&mut self, x: &Float, y: &Float, w: &Float, h: &Float) {
        todo!("Ellipse render for LuminanceRenderer not implemented");
    }
    
    fn ellipse_in_TRect(&mut self, in_rect: &TinRect) {
        todo!("Ellipse render inside TinRect for LuminanceRenderer not implemented");
    }

    fn rect(&mut self, withRect: &TinRect) {
        todo!("rect method in LuminanceRenderer not supported yet");
    }

    fn rect_with_TRect(&mut self, withRect: &TinRect) {
        todo!("Rect render with TinRect for LuminanceRenderer not implemented");
        //if self.delegate.fill {
            //cg.fill(rect)
        //}
        //if self.delegate.stroke {
            //cg.stroke(rect)
        //}
    }


    // Draw line with previously set line width
    fn line(&mut self, x1: &Double, y1: &Double, x2: &Double, y2: &Double) {
        todo!("Line render for LuminanceRenderer not implemented");
    }
    
    
    // Set line width
    fn lineWidth(&mut self, width: &Double) {
        todo!();
        //cg.setLineWidth(CGFloat(width));
    }
    
    fn triangle(&mut self, x1: &Double, y1: &Double, x2: &Double, y2: &Double, x3: &Double, y3: &Double) {
        let point1 = TinPoint::new_from_coords(*x1 as Float, *y1 as Float);
        let point2 = TinPoint::new_from_coords(*x2 as Float, *y2 as Float);
        let point3 = TinPoint::new_from_coords(*x3 as Float, *y3 as Float);
        self.EnqueueShape(&Vec::from([point1, point2, point3]));
    }
    
    // MARK: - Path stuff
    
    fn pathBegin(&mut self) {
        self.path_started = true
    }
    
    
    fn pathVertex(&mut self, atPoint: &TinPoint) {
        self.path_vertices.push_back(MakeVertex(atPoint, &self.get_fill_color()));
    }


    // MARK: - Rendering cycle

    
    fn pathAddCurve(&mut self, to: &TinPoint, control1: &TinPoint, control2: &TinPoint) {
        todo!("pathAddCurve method in LuminanceRenderer not supported yet");
        //cg.addCurve(to: to, control1: control1, control2: control2)
    }
    
    
    
    
    
    
    
    fn pathEnd(&mut self) {
        let mut verts: Vec<TinVertex> = Vec::new();

        for v in &mut self.path_vertices {
            verts.push(*v);
        }

        self.shape_queue.push_back( MakeShapeFromVec(verts) ); 
    }
    
    fn roundedRect(&mut self, rect: &TinRect, xRadius: &Double, yRadius: &Double) {
        todo!("roundedRect method in LuminanceRenderer not supported yet");
        /* 
        let bezier = NSBezierPath(roundedRect: rect, xRadius: CGFloat(xRadius), yRadius: CGFloat(yRadius))
        if delegate.fill {
            bezier.fill()
        }
        if delegate.stroke {
            bezier.stroke()
        }
        */
    }
    
    
    // MARK: - Color state
    
    fn set_stroke_color(&mut self, red: &Double, green: &Double, blue: &Double, alpha: &Double) {
        //cg.setStrokeColor(red: r, green: g, blue: b, alpha: a);
        self.current_stroke_color = TinColor::new_from_rgba(*red, *green, *blue, *alpha);
    }
    
    
    
    
    
    fn set_fill_color(&mut self, red: &Double, green: &Double, blue: &Double, alpha: &Double) {
        //cg.setFillColor(red: r, green: g, blue: b, alpha: a);
        self.current_fill_color = TinColor::new_from_rgba(*red, *green, *blue, *alpha);
    }
    
    
    fn get_stroke_color(&self) -> TinColor {
        return self.current_stroke_color.clone()
    }
    
    
    fn get_fill_color(&self) -> TinColor {
        return self.current_fill_color.clone()
    }
    
    fn set_alpha(&mut self, alpha: &Double) {
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
    
    
    // MARK: - Context state
    
    fn pushState(&mut self) {
        self.save_state = true;
    }
    
    fn popState(&mut self) {
        self.save_state = false;
    }
    
    // MARK: - Transformations
    
    fn translate(&mut self, dx: &Double, dy: &Double) {
        todo!("Translate not supported")
        // Set an offset value that affects all vertices drawn in current state so far
    }
    
    
    fn rotate(&mut self, angle: &Double) {
        todo!("Rotate not supported")
        // Set an offset value that affects all vertices drawn in current state so far
    }
    
    
    fn scale(&mut self, amount: &Double) {
        todo!("Scale not supported")
        // Increase distance between vertices in each shape in state so far
    }
    
    
    // MARK: - Image
    
    fn image(&mut self, image: &TinImage, x: &Double, y: &Double) {
        todo!("image not supported");
        // self.image_with_size(image, x, y, image.width, image.height, false);
    }
    
    
    fn image_with_size(&mut self, image: &TinImage, x: &Double, y: &Double, width: &Double, height: &Double) {
        self.image_with_size_and_resize(image, x, y, width, height, false);
    }

    fn image_with_size_and_resize(&mut self, image: &TinImage, x: &Double, y: &Double, width: &Double, height: &Double, resize: bool) {
        todo!("image_with_size_and_resize not supported");
        // Draw image at point
    }

    // MARK: - Text
    
    fn text(&mut self, message: &String, font: &TinFont, x: &Double, y: &Double) {
        todo!("text method in LuminanceRenderer not supported yet");
        // Draw text at location with default size
    }

    fn get_background_color(&self) -> TinColor {
        self.current_background_color
    }
}


impl Default for LuminanceRenderer {
    fn default() -> Self {
        Self {
            shape_queue: Queue::<TinShape>::new(),

            save_state: false,
            saved_shape_queue: Queue::<TinShape>::new(),
            drawType: DrawType::Fill,
            //delegate: TinContext::init(),// Probably need to change this when the context is fully implemented
            current_fill_color: DEFAULT_COLOR_FILL,
            current_stroke_color: DEFAULT_COLOR_STROKE,
            current_background_color: DEFAULT_COLOR_BACKGROUND,
            useLayer: false,

            path_started: false,
            path_vertices: Queue::<TinVertex>::new()
        }
    }
}

