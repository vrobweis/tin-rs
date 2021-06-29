use crate::{backends::{TextRenderer, nannou::NannouBackend}, point::TPoint};

impl TextRenderer for NannouBackend {
    fn text(
        &mut self,
        message: &String,
        font: &crate::text::TinFont,
        center: crate::point::TinPoint,
        state: crate::context::DrawState,
    ) {
        let draw = self
            .get_draw()
            .scale(state.scale as f32)
            .translate(nannou::prelude::Vector3::<f32>::new(
                state.translation.0 as f32,
                state.translation.1 as f32,
                0.0,
            ))
            .rotate(state.rotation as f32);
        let d = draw
            .text(message)
            .x_y(center.get_x() as f32, center.get_y() as f32);
        todo!("text method in NannouBackend not supported yet");
        // Draw text at location with default size
    }
}
