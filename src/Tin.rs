use std::sync::{Mutex, MutexGuard};

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

#[path = "Tin/Calculation.rs"] pub mod Calculation;
#[path = "Tin/DateAndTime.rs"] pub mod DateAndTime;
#[path = "Tin/Easing.rs"] pub mod Easing;
#[path = "Tin/ImprovedNoise.rs"]pub mod ImprovedNoise;
#[path = "Tin/TColor.rs"] pub mod TColor;
#[path = "Tin/TController.rs"] pub mod TController;
#[path = "Tin/TFont.rs"] pub mod TFont;
#[path = "Tin/TFrame.rs"] pub mod TFrame;
#[path = "Tin/TImage.rs"] pub mod TImage;
#[path = "Tin/TPoint.rs"] pub mod TPoint;
#[path = "Tin/TRandom.rs"] pub mod TRandom; //TODO: Implement TRandom
#[path = "Tin/LuminanceRenderer.rs"] pub mod LuminanceRenderer;
#[path = "Tin/TScene.rs"] pub mod TScene;
#[path = "Tin/TShapes.rs"] pub mod TShapes;
#[path = "Tin/TStopwatch.rs"] pub mod TStopwatch;
#[path = "Tin/TVector2.rs"] pub mod TVector2;
#[path = "Tin/TVertex.rs"] pub mod TVertex;
#[path = "Tin/TView.rs"] pub mod TView;

pub type Double = f64;
pub type Float = f32;

use lazy_static::lazy_static;

use TColor::{DEFAULT_COLOR_FILL, DEFAULT_COLOR_STROKE};

use LuminanceRenderer::LuminanceRenderer as LumRenderer;

use TColor::TinColor;
use TFont::TinFont; 
use TImage::TinImage;
use TShapes::TinRect;
use TFrame::TinFrame; 
use TPoint::TinPoint;
use luminance_glfw::GlfwSurface;
use luminance_windowing::{WindowDim, WindowOpt};

use self::TColor::DEFAULT_COLOR_BACKGROUND;
// TinImage::TinImage };




lazy_static! {
    static ref TIN_MUTEX: Mutex<TinContext> = Mutex::new(TinContext::new(Box::new(LumRenderer::default())));
}



pub fn get_tin<'t>() -> MutexGuard<'t, TinContext> {
    eprintln!("get_tin()");
    TIN_MUTEX.lock().unwrap()
}


pub trait TinRenderer {
    
    //let useLayer: bool { get set }
    
    //let delegate: TinContext { get set }
    
    // rendering setup
    fn prepare(&mut self, frame: TinFrame);
    
    
    // rendering cycle
    fn prepareForUpdate(&mut self, frame: TinFrame);
    fn didFinishUpdate(&mut self);
    
    
    // drawing methods
    fn background(&mut self, red: &Double, green: &Double, blue: &Double);
    
    fn arc(&mut self, x: &Double, y: &Double, radius: &Double, startAngle: &Double, endAngle: &Double);
    
    fn ellipse(&mut self, x: &Float, y: &Float, w: &Float, h: &Float);
    fn ellipse_in_TRect(&mut self, in_rect: &TinRect);

    fn rect(&mut self, x: &Double, y: &Double, w: &Double, h: &Double);
    fn rect_with_TRect(&mut self, withRect: &TinRect);
    
    fn line(&mut self, x1: &Double, y1: &Double, x2: &Double, y2: &Double);
    fn lineWidth(&mut self, width: &Double);
    fn triangle(&mut self, x1: &Double, y1: &Double, x2: &Double, y2: &Double, x3: &Double, y3: &Double);
    
    fn pathBegin(&mut self);
    fn pathVertex(&mut self, atPoint: &TinPoint);
    fn pathAddCurve(&mut self, to: &TinPoint, control1: &TinPoint, control2: &TinPoint);
    fn pathEnd(&mut self);
    
    fn roundedRect(&mut self, rect: &TinRect, xRadius: &Double, yRadius: &Double);
    
    
    // color state
    fn set_stroke_color(&mut self, red: &Double, green: &Double, blue: &Double, alpha: &Double);
    fn set_fill_color(&mut self, red: &Double, green: &Double, blue: &Double, alpha: &Double);
    fn get_stroke_color(&self) -> TinColor;
    fn get_fill_color(&self) -> TinColor;
    fn set_alpha(&mut self, alpha: &Double);

    fn get_background_color(&self) -> TinColor;
    
    
    // context state & transformations
    fn pushState(&mut self);
    fn popState(&mut self);
    fn translate(&mut self, dx: &Double, dy: &Double);
    fn rotate(&mut self, angle: &Double);
    fn scale(&mut self, amount: &Double);
    
    
    // image & text
    fn image(&mut self, image: &TinImage, x: &Double, y: &Double);
    fn image_with_size(&mut self, image: &TinImage, x: &Double, y: &Double, width: &Double, height: &Double);
    fn image_with_size_and_resize(&mut self, image: &TinImage, x: &Double, y: &Double, width: &Double, height: &Double, resize: bool);
    fn text(&mut self, message: &String, font: &TinFont, x: &Double, y: &Double); // TODO: Implement TFont
    
}


pub struct TinContext {
    
    pub fill: bool,
    pub stroke: bool,
    pub size: [Double;2],
    pub width: Double,
    pub height: Double,
    pub midX: Double,
    pub midY: Double,
    pub mouseX: Double,
    pub mouseY: Double,
    pub pmouseX: Double,
    pub pmouseY: Double,
    pub mousePressed: bool,
    pub frameCount: u16,

    current_fill_color: TinColor,
    current_stroke_color: TinColor,
    current_background_color: TinColor,

    pathVertexCount: u16,
    
    pub render: Box<LumRenderer>
}

impl TinContext {
    
    /// TODO: Document this method.
    pub fn new(renderer: Box<LumRenderer>) -> Self {
        Self {
            fill: true,
            stroke: true,
            size: [0.,0.],
            width: 0.0,
            height: 0.0,
            midX: 0.0,
            midY: 0.0,
            mouseX: 0.0,
            mouseY: 0.0,
            pmouseX: 0.0,
            pmouseY: 0.0,
            mousePressed: false,
            frameCount: 0,
            

            current_fill_color: DEFAULT_COLOR_FILL,
            current_stroke_color: DEFAULT_COLOR_STROKE,
            current_background_color: DEFAULT_COLOR_BACKGROUND,

            pathVertexCount: 0,
            
            render: renderer
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
    pub fn prepareForUpdate(&mut self, frame: TinFrame) {
        self.render.prepareForUpdate(frame);
    }
    
    /// TODO: Document this method.
    pub fn didFinishUpdate(&mut self) {
        self.render.didFinishUpdate();
    }
    
    /// TODO: Document this method.
    pub fn resetSize(&mut self, width: Double, height: Double) {
        self.size = [width, height];
        self.width = width;
        self.height = height;
        self.midX = width / 2.0;
        self.midY = height / 2.0;
    }
    

    /**
      Reset variables needed to draw next frame:
            - buffer size should match the window size
            - fill and stroke colors
            - line width
    */
    pub fn reset(&mut self, width: Double, height: Double) {
        self.resetSize(width, height);
        self.fill = true;
        self.stroke = true;
        lineWidth(&0.05);
        
        fillColor_from_color(&DEFAULT_COLOR_FILL);
        strokeColor_from_color(&DEFAULT_COLOR_STROKE);
    }
    
    
/* 

    // enable "Restore from previous"
    // When enabled, this feature will cause the rendering to be saved in an image buffer.
    // After the double buffer swap, before the next render happens, the image saved
    // in the buffer is restored to the current frame buffer. This allows continuous draw effects.
    // Note, there is a time penalty for saving and restoring the image.
    pub fn enableRestoreFromPrevious(&mut self, ) {
        self.render.useLayer = true
    }
    
    
    pub fn disableRestoreFromPrevious(&mut self, ) {
        self.render.useLayer = false
    }
    */
    

    /// TODO: Document this function.
    pub fn mouseMoved(&mut self, toPoint: TinPoint) {
        self.pmouseX = self.mouseX;
        self.pmouseY = self.mouseY;
        self.mouseX = toPoint.x as Double;
        self.mouseY = toPoint.y as Double;
    }
    
    /// TODO: Document this function.
    pub fn updateFrameCount(&mut self) {
        self.frameCount += 1;
    }
    
}


// MARK: - Global drawing methods

#[allow(dead_code)]
/// Clear (erase) the background
pub fn background(red: Double, green: Double, blue: Double) {
    let mut tin = get_tin();
    tin.render.background(&red, &green, &blue);
}

#[allow(dead_code)]
/// TODO: Document this function.
pub fn background_gray(gray: Double) {
    let mut tin = get_tin();
    tin.render.background(&gray, &gray, &gray);
}

#[allow(dead_code)]
/// TODO: Document this function.
pub fn background_with_tinColor(color: &TinColor) {
    let mut tin = get_tin();
    tin.render.background( &color.red, &color.green, &color.blue);
}

#[allow(dead_code)]
/// TODO: Document this function.
pub fn arc(x: &Double, y: &Double, radius: &Double, startAngle: &Double, endAngle: &Double) {
    let mut tin = get_tin();
    tin.render.arc( x, y, radius, startAngle, endAngle);
}


// Ellipse method
#[allow(dead_code)]
/// Draw an ellipse. Input is centerX, centerY coordinate and width, height size.
pub fn ellipse(centerX: &Double, centerY: &Double, width: &Double, height: &Double) {
    let x = centerX - width / 2.0;
    let y = centerY - height / 2.0;
    let r = TinRect::new_from_dimensions( x, y, *width, *height);
    let mut tin = get_tin();
    tin.render.ellipse_in_TRect( &r);
}


// Line methods
#[allow(dead_code)]
/// TODO: Document this function.
pub fn line(x1: &Double, y1: &Double, x2: &Double, y2: &Double) {
    let mut tin = get_tin();
    tin.render.line( x1, y1, x2, y2);
}

#[allow(dead_code)]
/// TODO: Document this function.
pub fn lineWidth(width: &Double) {
    let mut tin = get_tin();
    tin.render.lineWidth(width);
}


// Rectangle method
#[allow(dead_code)]
/// Draw a rectangle. Input is left, bottom coordinate and width, height size.
pub fn rect(x: &Double, y: &Double, width: &Double, height: &Double) {
    eprintln!("Tin::rect()");
    let r = TinRect::new_from_dimensions( *x, *y, *width, *height);
    let mut tin = get_tin();
    tin.render.rect_with_TRect(&r);
}

#[allow(dead_code)]
/// TODO: Document this function.
pub fn triangle(x1: Double, y1: Double, x2: Double, y2: Double, x3: Double, y3: Double) {
    eprintln!("Tin::triangle()");
    let mut tin = get_tin();
    tin.render.triangle(&x1, &y1, &x2, &y2, &x3, &y3);
}


// Path methods

#[allow(dead_code)]
/// Create a new path.
pub fn pathBegin() {
    let mut tin = get_tin();
    tin.render.pathBegin();
    tin.pathVertexCount = 0;
}

#[allow(dead_code)]
/// Add a new point to the current path. (input 2 CGFloats)
pub fn pathVertex(x: &Double, y: &Double) {
    let point = TinPoint {x: *x, y: *y};
    let mut tin = get_tin();
    tin.render.pathVertex( &point);
    tin.pathVertexCount += 1;
}

#[allow(dead_code)]
/// Add a bezier curve to the current path
pub fn pathAddCurve(to: &TinPoint, control1: &TinPoint, control2: &TinPoint) {
    let mut tin = get_tin();
    tin.render.pathAddCurve( to, control1, control2);
    tin.pathVertexCount += 4;
}

#[allow(dead_code)]
/// Stroke/Fill the current path.
pub fn pathEnd() {
    let mut tin = get_tin();
    tin.render.pathEnd();
    tin.pathVertexCount = 0;
}

#[allow(dead_code)]
/// Draw a rectangle with rounded corners, specified by xRadius, yRadius
pub fn roundedRect(rect: &TinRect, xRadius: &Double, yRadius: &Double) {
    let mut tin = get_tin();
    tin.render.roundedRect(rect, xRadius, yRadius);
}


// MARK: - Color state
#[allow(dead_code)]
/// TODO: Document this function.
pub fn strokeColor_from_RGBA(red: &Double, green: &Double, blue: &Double, alpha: &Double) {
    let mut tin = get_tin();
    tin.stroke = true;
    tin.render.set_stroke_color(red, green, blue, alpha);
}

#[allow(dead_code)]
/// TODO: Document this function.
pub fn strokeColor_from_grayAndAlpha(gray: &Double, alpha: &Double) {
    let mut tin = get_tin();
    tin.stroke = true;
    tin.render.set_stroke_color(gray, gray, gray, alpha);
}

#[allow(dead_code)]
/// TODO: Document this function.
pub fn strokeColor_from_gray(gray: &Double) {
    let mut tin = get_tin();
    tin.stroke = true;
    tin.render.set_stroke_color(gray, gray, gray, &1.0);
}

#[allow(dead_code)]
/// TODO: Document this function.
pub fn strokeColor_from_color(color: &TinColor) {
    let mut tin = get_tin();
    tin.stroke = true;
    tin.render.set_stroke_color( &color.red, &color.green, &color.blue, &color.alpha);
}

#[allow(dead_code)]
/// TODO: Document this function.
pub fn fillColor_from_RGBA(red: &Double, green: &Double, blue: &Double, alpha: &Double) {
    let mut tin = get_tin();
    tin.fill = true;
    tin.render.set_fill_color( red, green, blue, alpha);
}

#[allow(dead_code)]
/// TODO: Document this function.
pub fn fillColor_from_grayAndAlpha(gray: &Double, alpha: &Double) {
    let mut tin = get_tin();
    tin.fill = true;
    tin.render.set_fill_color( gray, gray, gray, alpha);
}

#[allow(dead_code)]
/// TODO: Document this function.
pub fn fillColor_from_gray(gray: &Double) {
    let mut tin = get_tin();
    tin.fill = true;
    tin.render.set_fill_color(gray, gray, gray, &1.0);
}

#[allow(dead_code)]
/// TODO: Document this function.
pub fn fillColor_from_color(color: &TinColor) {
    let mut tin = get_tin();
    tin.fill = true;
    tin.render.set_fill_color( &color.red, &color.green, &color.blue, &color.alpha);
}

#[allow(dead_code)]
/// TODO: Document this function.
pub fn strokeColor() -> TinColor {
    let tin = get_tin();
    return tin.render.get_stroke_color();
}

#[allow(dead_code)]
/// TODO: Document this function.
pub fn fillColor() -> TinColor {
    let tin = get_tin();
    return tin.render.get_fill_color();
}

#[allow(dead_code)]
/// TODO: Document this function.
pub fn setAlpha(alpha: &Double) {
    let mut tin = get_tin();
    tin.render.set_alpha(alpha);
}

#[allow(dead_code)]
/// TODO: Document this function.
pub fn strokeDisable() {
    let mut tin = get_tin();
    tin.stroke = false;
}

#[allow(dead_code)]
/// TODO: Document this function.
pub fn fillDisable() {
    let mut tin = get_tin();
    tin.fill = false;
}

#[allow(dead_code)]
/// TODO: Document this function.
pub fn strokeEnable() {
    let mut tin = get_tin();
    tin.stroke = true;
}

#[allow(dead_code)]
/// TODO: Document this function.
pub fn fillEnable() {
    let mut tin = get_tin();
    tin.fill = true;
}


// MARK: - Context state and Transformations

#[allow(dead_code)]
/// TODO: Document this function.
pub fn pushState() {
    let mut tin = get_tin();
    tin.render.pushState();
}

#[allow(dead_code)]
/// TODO: Document this function.
pub fn popState() {
    let mut tin = get_tin();
    tin.render.popState();
}

#[allow(dead_code)]
/// TODO: Document this function.
pub fn translate(dx: &Double, dy: &Double) {
    let mut tin = get_tin();
    tin.render.translate(dx, dy);
}

#[allow(dead_code)]
/// TODO: Document this function.
pub fn rotate(byAngle: &Double) {
    let mut tin = get_tin();
    tin.render.rotate(byAngle);
}

#[allow(dead_code)]
/// TODO: Document this function.
pub fn scale(amount: &Double) {
    let mut tin = get_tin();
    tin.render.scale(amount);
}


// MARK: - Image

#[allow(dead_code)]
/// TODO: Document this function.
pub fn image(image: &TinImage, x: &Double, y: &Double) {
    let mut tin = get_tin();
    tin.render.image(image, x, y);
}

#[allow(dead_code)]
/// TODO: Document this function.
pub fn image_with_size(image: &TinImage, x: &Double, y: &Double, width: &Double, height: &Double) {
    let mut tin = get_tin();
    tin.render.image_with_size(image, x, y, width, height);
}

#[allow(dead_code)]
/// TODO: Document this function.
pub fn image_with_size_and_resize(image: &TinImage, x: &Double, y: &Double, width: &Double, height: &Double, resize: bool) {
    let mut tin = get_tin();
    tin.render.image_with_size_and_resize(image, x, y, width, height, resize);
    // Draw image at point
}


// MARK: - Text
/*
pub fn text(message: String, font: TFont, x: Double, y: Double) {
    tin.render.text(message, font, x, y)
}
*/
