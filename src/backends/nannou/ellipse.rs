use crate::{backends::{EllipseRenderer, nannou::NannouBackend}, point::TPoint};

impl EllipseRenderer for NannouBackend {
    fn ellipse(
        &mut self,
        center: crate::point::TinPoint,
        w: crate::Double,
        h: crate::Double,
        brush: crate::brush::TBrush,
        state: crate::context::DrawState,
    ) {
        eprintln!("NannouBackend::ellipse_in_tinrect()");

        let draw = nannou::prelude::Draw::new()
            .scale(state.scale as f32)
            .translate(nannou::prelude::Vector3::<f32>::new(
                state.translation.0 as f32,
                state.translation.1 as f32,
                0.0,
            ))
            .rotate(state.rotation as f32);

        let d = draw
            .ellipse()
            .x_y(center.get_x() as f32, center.get_x() as f32)
            .w(w as f32)
            .h(h as f32);

        crate::draw_with_brush!(d, brush);
        todo!("Ellipse render inside TinRect for NannouBackend not implemented");
    }
}
