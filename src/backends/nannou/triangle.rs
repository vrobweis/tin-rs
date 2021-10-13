use crate::{backends::{TriangleRenderer, nannou::NannouBackend}, draw_with_brush, vector2::TinVector2};

impl TriangleRenderer for NannouBackend {
    fn triangle(
        &mut self,
        triangle: crate::shapes::TinTriangle,
        brush: crate::brush::TBrush,
        state: crate::context::DrawState,
    ) {
        let vector1 = TinVector2::from(triangle.point1);
        let vector2 = TinVector2::from(triangle.point2);
        let vector3 = TinVector2::from(triangle.point3);

        let draw = nannou::prelude::Draw::new()
            .scale(state.scale as f32)
            .translate(nannou::prelude::vec3(
                state.translation.0 as f32,
                state.translation.1 as f32,
                0.0,
            ))
            .rotate(state.rotation as f32);

        let d = draw.tri().points(vector1, vector2, vector3);

        draw_with_brush!(d, brush);
    }
}
