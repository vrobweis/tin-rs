use crate::{Float, backends::EllipseRenderer, shapes::TinRect};

use super::NannouBackend;



impl EllipseRenderer for NannouBackend {
    fn ellipse(&mut self, x: Float, y: Float, w: Float, h: Float) {
        eprintln!("NannouBackend::ellipse()");
        todo!("Ellipse render for NannouBackend not implemented");
    }
    
    fn ellipse_in_tinrect(&mut self, in_rect: &TinRect) {
        eprintln!("NannouBackend::ellipse_in_tinrect()");
        todo!("Ellipse render inside TinRect for NannouBackend not implemented");
    }
}