use crate::backends::ArcRenderer;

use super::NannouBackend;

impl ArcRenderer for NannouBackend {
    fn arc(
        &mut self,
        arc: crate::shapes::TinArc,
        brush: crate::brush::TBrush,
        state: crate::context::DrawState,
    ) {
        eprintln!("NannouBackend::arc()");
        todo!("Arc render for NannouBackend not implemented");
    }
}
