use crate::{brush::TBrush, context::DrawState, point::TinPoint, shapes::rect::TinRect, Double};

pub(crate) trait EllipseRenderer {
    fn ellipse(&mut self, center: TinPoint, w: Double, h: Double, brush: TBrush, state: DrawState);
    fn ellipse_in_tinrect(&mut self, in_rect: &TinRect, brush: TBrush, state: DrawState) {
        self.ellipse(
            in_rect.center.clone(),
            in_rect.get_width(),
            in_rect.get_height(),
            brush,
            state,
        );
    }
}
