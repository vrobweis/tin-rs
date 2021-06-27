use crate::{
    backends::{luminance::LuminanceBackend, TriangleRenderer},
    context::DrawState,
    shapes::TinTriangle,
    vector2::TinVector2,
};

impl TriangleRenderer for LuminanceBackend {
    fn triangle(&mut self, triangle: TinTriangle, brush: crate::brush::TBrush, state: DrawState) {
        let vector1 = TinVector2::from(triangle.point1);
        let vector2 = TinVector2::from(triangle.point2);
        let vector3 = TinVector2::from(triangle.point3);
        self.enqueue_shape(Vec::from([vector1, vector2, vector3]), brush, state);
    }
}
