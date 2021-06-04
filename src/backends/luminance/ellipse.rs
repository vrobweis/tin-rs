use crate::{
    Double, 
    backends::{EllipseRenderer, luminance::LuminanceBackend}, 
    context::{DrawState, TBrush}, 
    shapes::TinRect
};

impl EllipseRenderer for LuminanceBackend {

    fn ellipse(&mut self, x: &Double, y: &Double, w: &Double, h: &Double, brush: TBrush, state: DrawState) {
        todo!("Ellipse render for LuminanceBackend not implemented");
    }

}

