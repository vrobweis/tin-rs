use crate::{
    backends::{luminance::LuminanceBackend, EllipseRenderer},
    context::DrawState,
    Double,
};

impl EllipseRenderer for LuminanceBackend {
    fn ellipse(
        &mut self,
        center: crate::point::TinPoint,
        w: Double,
        h: Double,
        brush: crate::brush::TBrush,
        state: DrawState,
    ) {
        todo!("Ellipse render for LuminanceBackend not implemented");
    }
}
