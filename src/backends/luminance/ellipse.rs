use crate::{Double, backends::EllipseRenderer, context::{DrawState, TBrush}, shapes::TinRect};

use super::LuminanceBackend;



impl EllipseRenderer for LuminanceBackend {
    fn ellipse(&mut self, x: &Double, y: &Double, w: &Double, h: &Double, brush: TBrush, state: DrawState) {
        eprintln!("LuminanceBackend::ellipse()");
        todo!("Ellipse render for LuminanceBackend not implemented");
    }
    
    fn ellipse_in_tinrect(&mut self, in_rect: &TinRect, brush: TBrush, state: DrawState) {
        eprintln!("LuminanceBackend::ellipse_in_tinrect()");
        self.ellipse(&in_rect.x, &in_rect.y, &in_rect.get_width(), &in_rect.get_height(), brush, state);
    }
}