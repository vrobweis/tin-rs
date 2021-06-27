use crate::{backends::RectRenderer, point::TPoint, shapes::TinRect};

use super::NannouBackend;

impl RectRenderer for NannouBackend {
    fn rounded_rect(
        &mut self,
        rounded_rect: &crate::shapes::TinRoundedRect,
        brush: crate::brush::TBrush,
        state: crate::context::DrawState,
    ) {
        todo!("rounded_rect method in NannouBackend not supported yet");
    }

    fn rect_with_tinrect(
        &mut self,
        with_rect: &TinRect,
        brush: crate::brush::TBrush,
        state: crate::context::DrawState,
    ) {
        let center = &with_rect.center;
        let draw = nannou::prelude::Draw::new()
            .scale(state.scale as f32)
            .translate(nannou::prelude::Vector3::<f32>::new(
                state.translation.0 as f32,
                state.translation.1 as f32,
                0.0,
            ))
            .rotate(state.rotation as f32);
        let rect = draw
            .rect()
            .w_h(with_rect.get_width() as f32, with_rect.get_height() as f32)
            .x_y(center.get_x() as f32, center.get_y() as f32);

        crate::draw_with_brush!(rect, brush);
    }
}
