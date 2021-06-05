pub(crate) mod luminance;
// TODO: Implement alternative, potentially more performant backends

// pub(crate) mod nannou; 

use crate::{Double, Float, TinApp, UInt, ULong, UShort, context::{DrawState, TBrush}, font::TinFont, frame::TinFrame, image::TinImage, point::{TPoint}, scene::TScene, shapes::TinRect};


pub(crate) trait TBackend: TinRenderer {
    fn new() -> Self;

    fn run<S>(app: TinApp<S>) -> Result<(), ()> where S: TScene ;
}

pub(crate) trait TinRenderer: /*BackgroundRenderer +*/ RectRenderer + TriangleRenderer 
    + LineRenderer + ArcRenderer + EllipseRenderer + ImageRenderer
    + PathRenderer + StatefulRenderer
{
    
    /// Handle rendering setup.
    fn prepare(&mut self, _frame: TinFrame){}
    
    
    // MARK: rendering cycle
    fn prepare_for_update(&mut self);
    fn did_finish_update(&mut self);
    
    
}

pub(crate) trait RectRenderer {
    fn rect(&mut self, x: Double, y: Double, w: Double, h: Double, brush: TBrush, state: DrawState) {    
        self.rect_with_tinrect(&TinRect::new_from_dimensions(x, y, w, h), brush, state);
    }
    fn rect_with_tinrect(&mut self, with_rect: &TinRect, brush: TBrush, state: DrawState);
    fn rounded_rect(&mut self, rect: &TinRect, radius_x: Double, radius_y: Double, brush: TBrush, state: DrawState);
}

pub(crate) trait TriangleRenderer {
    fn triangle(&mut self, x1: Double, y1: Double, x2: Double, y2: Double, x3: Double, y3: Double, brush: TBrush, state: DrawState);
}
pub(crate) trait LineRenderer {
    fn line(&mut self, x1: Double, y1: Double, x2: Double, y2: Double, width: Double, brush: TBrush, state: DrawState);
}

pub(crate) trait ArcRenderer {
    fn arc(&mut self, x: Double, y: Double, radius: Double, start_angle: Double, end_angle: Double, brush: TBrush, state: DrawState);
}

pub(crate) trait EllipseRenderer {
    fn ellipse(&mut self, x: Double, y: Double, w: Double, h: Double, brush: TBrush, state: DrawState);
    fn ellipse_in_tinrect(&mut self, in_rect: &TinRect, brush: TBrush, state: DrawState) {
        self.ellipse(in_rect.x, in_rect.y, in_rect.get_width(), in_rect.get_height(), brush, state);
    }
}

pub(crate) trait PathRenderer {
    fn path_begin(&mut self);
    fn path_vertex(&mut self, at_point: &impl TPoint);
    fn path_add_curve(&mut self, to: &impl TPoint, control1: &impl TPoint, control2: &impl TPoint);
    fn path_end(&mut self);
}

pub(crate) trait ImageRenderer {
    fn image(&mut self, image: &TinImage, x: Double, y: Double) {
        self.image_with_size(image, x, y, image.width as Double, image.height as Double)
    }
    
    
    fn image_with_size(&mut self, image: &TinImage, x: Double, y: Double, width: Double, height: Double) {
        self.image_with_size_and_resize(image, x, y, width, height, false);
    }

    fn image_with_size_and_resize(&mut self, image: &TinImage, x: Double, y: Double, width: Double, height: Double, resize: bool);
}

pub(crate) trait StatefulRenderer {
    fn push_state(&mut self);
    fn pop_state(&mut self);

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

}


pub(crate) trait TextRenderer {
    fn text(&mut self, message: &String, font: &TinFont, x: Double, y: Double); // TODO: Implement TFont
}

use std::{
    time::{Instant, Duration},
    thread::sleep,
};
pub(crate) fn pace_frames(target_fps: UShort, last_frame_time: Instant) {
    let frametime_ms: UInt = match target_fps as UInt {
        // Prevent divide by zero
        0 => 1000 / 1,
        _ => 1000 / target_fps as UInt
    };
    
    let safe_sleep_period_ms = frametime_ms as Float * (9_f32 / 10_f32);
    let safe_sleep_period: Duration = Duration::from_millis(safe_sleep_period_ms as ULong);

    // TODO: Use stopwatch.rs methods to replicate this functionality
    
    if Instant::now().duration_since(last_frame_time).as_millis() < safe_sleep_period.as_millis()
    {
        sleep(safe_sleep_period);
    }

    while (Instant::now().duration_since(last_frame_time).as_millis() as UInt) < frametime_ms {
        // Spinlock for whatever time is left after the thread sleep function
    }
}