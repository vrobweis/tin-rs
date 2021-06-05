use crate::{Double, backends::RectRenderer, point::TinPoint, shapes::TinRect, vector2::TVector2};

use super::NannouBackend;



impl RectRenderer for NannouBackend {
    fn rect(&mut self, x: Double, y: Double, w: Double, h: Double) {    
        eprintln!("NannouBackend::rect()");
        self.rect_with_tinrect(&TinRect::new_from_dimensions(*x, *y, *w, *h));
    }

    fn rect_with_tinrect(&mut self, with_rect: &TinRect) {
        eprintln!("NannouBackend::rect_with_tinrect()");
        let bottom_left_point = TinPoint::new_from_coords(with_rect.x, with_rect.y);

        let width = with_rect.get_width();
        let height = with_rect.get_height();

        let point1 = TVector2::new_from_xy(bottom_left_point.x, bottom_left_point.y);
        let point2 = TVector2::new_from_xy(bottom_left_point.x, bottom_left_point.y + height);
        let point3 = TVector2::new_from_xy(bottom_left_point.x + width, bottom_left_point.y + height);
        let point4 = TVector2::new_from_xy(bottom_left_point.x + width, bottom_left_point.y);

        self.enqueue_shape(Vec::from([point1.clone(),point2,point3,point4,point1]));
    }
    
    fn rounded_rect(&mut self, rect: &TinRect, radius_x: Double, radius_y: Double) {
        todo!("rounded_rect method in NannouBackend not supported yet");
        /* 
        let bezier = NSBezierPath(rounded_rect: rect, radius_x: CGFloat(radius_x), radius_y: CGFloat(radius_y))
        if delegate.fill {
            bezier.fill()
        }
        if delegate.stroke {
            bezier.stroke()
        }
        */
    }
}