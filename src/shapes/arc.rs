use crate::{brush::TBrush, context::DrawState, point::TinPoint, Double};

#[derive(Debug, Clone)]
pub(crate) struct TinArc {
    pub center: TinPoint,
    pub radius: Double,
    pub start_angle: Double,
    pub end_angle: Double,
}

impl TinArc {
    pub fn new(center: TinPoint, radius: Double, start_angle: Double, end_angle: Double) -> Self {
        Self {
            center,
            radius,
            start_angle,
            end_angle,
        }
    }
}

pub(crate) trait ArcRenderer {
    fn arc(&mut self, arc: TinArc, brush: TBrush, state: DrawState);
}
