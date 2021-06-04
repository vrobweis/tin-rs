use crate::{Double, backends::TriangleRenderer, context::{DrawState, TBrush}, vector2::TVector2};

use super::LuminanceBackend;


impl TriangleRenderer for LuminanceBackend {
    fn triangle(&mut self, x1: &Double, y1: &Double, x2: &Double, y2: &Double, x3: &Double, y3: &Double, brush: TBrush, state: DrawState) {
        let point1 = TVector2::new_from_xy(*x1, *y1);
        let point2 = TVector2::new_from_xy(*x2, *y2);
        let point3 = TVector2::new_from_xy(*x3, *y3);
        self.enqueue_shape(Vec::from([point1, point2, point3]), brush, state);
    }
}