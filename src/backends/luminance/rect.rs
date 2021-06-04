use crate::{
    Double, 
    backends::{RectRenderer, luminance::LuminanceBackend}, 
    context::{DrawState, TBrush}, 
    point::TinPoint, 
    shapes::TinRect, 
    vector2::TVector2
};


impl RectRenderer for LuminanceBackend {

    fn rect_with_tinrect(&mut self, with_rect: &TinRect, brush: TBrush, state: DrawState) {
        let bottom_left_point = TinPoint::new_from_coords(with_rect.x, with_rect.y);

        let width = with_rect.get_width();
        let height = with_rect.get_height();

        let point1 = TVector2::new_from_xy(bottom_left_point.x, bottom_left_point.y);
        let point2 = TVector2::new_from_xy(bottom_left_point.x, bottom_left_point.y + height);
        let point3 = TVector2::new_from_xy(bottom_left_point.x + width, bottom_left_point.y + height);
        let point4 = TVector2::new_from_xy(bottom_left_point.x + width, bottom_left_point.y);

        self.enqueue_shape(Vec::from([point1.clone(),point2,point3,point4,point1]), brush, state);
    }

    fn rounded_rect(&mut self, rect: &TinRect, radius_x: &Double, radius_y: &Double, brush: TBrush, state: DrawState) {
        todo!("rounded_rect method in LuminanceBackend not supported yet");
    }
}