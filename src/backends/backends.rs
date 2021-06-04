pub(crate) mod luminance;
//pub(crate) mod nannou;

use std::time::{Instant, Duration};

use crate::{context::{DrawState, TBrush}, event::TinEvent, scene::TScene, view::TinView};

use super::{Double, controller::TinController, font::TinFont, frame::TinFrame, image::TinImage, point::TinPoint, shapes::TinRect};


pub(crate) trait TBackend: TinRenderer { // TODO: This should not be a public trait
    // TODO: define functions needed to run a graphics backend
    fn new() -> Self;

    fn run<S, H>(controller: TinController<S,H>) -> Result<(), ()> where S: TScene, H: Fn(TinEvent, &mut S, &mut TinView) ;
}

pub(crate) trait TinRenderer: /*BackgroundRenderer +*/ RectRenderer + TriangleRenderer 
    + LineRenderer + ArcRenderer + EllipseRenderer + ImageRenderer
    + PathRenderer + StatefulRenderer
{
    
    //let use_layer: bool { get set }
    
    //let delegate: TinContext { get set }
    
    // rendering setup
    fn prepare(&mut self, _frame: TinFrame){}
    
    
    // rendering cycle
    fn prepare_for_update(&mut self);
    fn did_finish_update(&mut self);
    

    // color state
    /*
    fn set_stroke_color(&mut self, red: &Double, green: &Double, blue: &Double, alpha: &Double);
    fn set_fill_color(&mut self, red: &Double, green: &Double, blue: &Double, alpha: &Double);
    fn get_stroke_color(&self) -> TinColor;
    fn get_fill_color(&self) -> TinColor;
    fn set_alpha(&mut self, alpha: &Double);

    fn get_background_color(&self) -> TinColor;
    */
    
}

pub(crate) trait RectRenderer {
    fn rect(&mut self, x: &Double, y: &Double, w: &Double, h: &Double, brush: TBrush, state: DrawState);
    fn rect_with_tinrect(&mut self, with_rect: &TinRect, brush: TBrush, state: DrawState);
    fn rounded_rect(&mut self, rect: &TinRect, radius_x: &Double, radius_y: &Double, brush: TBrush, state: DrawState);
}

pub(crate) trait TriangleRenderer {
    fn triangle(&mut self, x1: &Double, y1: &Double, x2: &Double, y2: &Double, x3: &Double, y3: &Double, brush: TBrush, state: DrawState);
}
pub(crate) trait LineRenderer {
    fn line(&mut self, x1: &Double, y1: &Double, x2: &Double, y2: &Double, width: &Double, brush: TBrush, state: DrawState);
}

pub(crate) trait ArcRenderer {
    fn arc(&mut self, x: &Double, y: &Double, radius: &Double, start_angle: &Double, end_angle: &Double, brush: TBrush, state: DrawState);
}

pub(crate) trait EllipseRenderer {
    fn ellipse(&mut self, x: &Double, y: &Double, w: &Double, h: &Double, brush: TBrush, state: DrawState);
    fn ellipse_in_tinrect(&mut self, in_rect: &TinRect, brush: TBrush, state: DrawState);
}

pub(crate) trait PathRenderer {
    fn path_begin(&mut self);
    fn path_vertex(&mut self, at_point: &TinPoint);
    fn path_add_curve(&mut self, to: &TinPoint, control1: &TinPoint, control2: &TinPoint);
    fn path_end(&mut self);
}

pub(crate) trait ImageRenderer {
    fn image(&mut self, image: &TinImage, x: &Double, y: &Double) {
        self.image_with_size(image, x, y, &(image.width as f64), &(image.height as f64))
    }
    
    
    fn image_with_size(&mut self, image: &TinImage, x: &Double, y: &Double, width: &Double, height: &Double) {
        self.image_with_size_and_resize(image, x, y, width, height, false);
    }

    fn image_with_size_and_resize(&mut self, image: &TinImage, x: &Double, y: &Double, width: &Double, height: &Double, resize: bool);
}

pub(crate) trait StatefulRenderer {
    fn push_state(&mut self);
    fn pop_state(&mut self);
}


pub(crate) trait TextRenderer {
    fn text(&mut self, message: &String, font: &TinFont, x: &Double, y: &Double); // TODO: Implement TFont
}


fn pace_frames(target_fps: u16, last_frame_time: Instant) {
    let mut target_fps_local: u32 = target_fps as u32;
    
    if target_fps_local == 0 {target_fps_local = 1};
    let frametime_ms: u32 = 1000 / target_fps_local;
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