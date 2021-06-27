use crate::{
    backends::luminance::LuminanceBackend,
    context::DrawState,
    point::{TPoint, TinPoint},
    shapes::{RectRenderer, TinRect, TinRoundedRect},
    vector2::TinVector2,
    Double,
};

impl RectRenderer for LuminanceBackend {
    fn rect_with_tinrect(
        &mut self,
        with_rect: &TinRect,
        brush: crate::brush::TBrush,
        state: DrawState,
    ) {
        let center = TinPoint::from_coords(with_rect.center.get_x(), with_rect.center.get_y());

        let width = with_rect.get_width();
        let height = with_rect.get_height();

        let w_offset = width / 2 as Double;
        let h_offset = height / 2 as Double;

        let bottom_left = TinVector2::from_xy(center.get_x() - w_offset, center.get_y() - h_offset);
        let top_left = TinVector2::from_xy(center.get_x() - w_offset, center.get_y() + h_offset);
        let top_right = TinVector2::from_xy(center.get_x() + w_offset, center.get_y() + h_offset);
        let bottom_right =
            TinVector2::from_xy(center.get_x() + w_offset, center.get_y() - h_offset);

        self.enqueue_shape(
            Vec::from([bottom_left, top_left, top_right, bottom_right, bottom_left]),
            brush,
            state,
        );
    }

    fn rounded_rect(
        &mut self,
        rounded_rect: &TinRoundedRect,
        brush: crate::brush::TBrush,
        state: DrawState,
    ) {
        todo!("rounded_rect method in LuminanceBackend not supported yet");
    }
}
