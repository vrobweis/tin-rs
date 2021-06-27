use crate::{brush::TBrush, context::DrawState, point::TinPoint};

#[derive(Debug, Clone)]
pub(crate) struct TinTriangle {
    pub point1: TinPoint,
    pub point2: TinPoint,
    pub point3: TinPoint,
}

impl TinTriangle {
    pub fn new(point1: TinPoint, point2: TinPoint, point3: TinPoint) -> Self {
        Self {
            point1,
            point2,
            point3,
        }
    }
}

pub(crate) trait TriangleRenderer {
    fn triangle(&mut self, triangle: crate::shapes::TinTriangle, brush: TBrush, state: DrawState);
}
