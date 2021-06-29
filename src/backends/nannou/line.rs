use crate::{backends::{LineRenderer, nannou::NannouBackend}, vector2::TinVector2, Double};

impl LineRenderer for NannouBackend {
    // Draw line with previously set line width
    fn line(
        &mut self,
        point1: crate::point::TinPoint,
        point2: crate::point::TinPoint,
        width: Double,
        brush: crate::brush::TBrush,
        state: crate::context::DrawState,
    ) {
        let tvector1 = TinVector2::from(point1);
        let tvector2 = TinVector2::from(point2);

        let vector1 = nannou::prelude::Vector2::from(tvector1);
        let vector2 = nannou::prelude::Vector2::from(tvector2);
        let draw = nannou::prelude::Draw::new()
            .scale(state.scale as f32)
            .translate(nannou::prelude::Vector3::<f32>::new(
                state.translation.0 as f32,
                state.translation.1 as f32,
                0.0,
            ))
            .rotate(state.rotation as f32);

        let d = draw.line().points(vector1, vector2).weight(width as f32);

        todo!("line method for Nannou backend not implemented");
    }
}
