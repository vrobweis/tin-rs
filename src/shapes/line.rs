use crate::{brush::TBrush, context::DrawState, point::TinPoint, Double};

pub(crate) trait LineRenderer {
    fn line(
        &mut self,
        point1: TinPoint,
        point2: TinPoint,
        width: Double,
        brush: TBrush,
        state: DrawState,
    );
}
