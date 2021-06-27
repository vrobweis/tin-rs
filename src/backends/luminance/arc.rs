use crate::{
    backends::{luminance::LuminanceBackend, ArcRenderer},
    context::DrawState,
    shapes::TinArc,
};

impl ArcRenderer for LuminanceBackend {
    fn arc(&mut self, arc: TinArc, brush: crate::brush::TBrush, state: DrawState) {
        todo!("Arc render for LuminanceBackend not implemented");
    }
}
