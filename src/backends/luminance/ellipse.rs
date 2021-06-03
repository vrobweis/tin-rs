use crate::{Double, backends::EllipseRenderer, context::{DrawState, DrawType}, shapes::TinRect};

use super::LuminanceBackend;



impl EllipseRenderer for LuminanceBackend {
    fn ellipse(&mut self, x: &Double, y: &Double, w: &Double, h: &Double, brush: DrawType, state: DrawState) {
        eprintln!("LuminanceBackend::ellipse()");
        todo!("Ellipse render for LuminanceBackend not implemented");
    }
    
    fn ellipse_in_tinrect(&mut self, in_rect: &TinRect, brush: DrawType, state: DrawState) {
        eprintln!("LuminanceBackend::ellipse_in_tinrect()");
        self.ellipse(&in_rect.x, &in_rect.y, &in_rect.get_width(), &in_rect.get_height(), brush, state);
    }
}