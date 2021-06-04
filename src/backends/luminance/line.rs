use crate::{
    Double, 
    backends::{luminance::LuminanceBackend,LineRenderer},
    context::{DrawState, TBrush}, 
    vector2::TVector2
};


impl LineRenderer for LuminanceBackend {

    /// TODO: Document this method.
    fn line(&mut self, x1: &Double, y1: &Double, x2: &Double, y2: &Double, width: &Double, brush: TBrush, state: DrawState) {
        let relative_width = width/2.0;
        let first_vector = TVector2::new_from_xy(*x1, *y1);
        let second_vector = TVector2::new_from_xy(*x2, *y2);

        let mut perpendicular_clockwise_vector;
        let mut perpendicular_counter_clockwise_vector;
        {
            let line_vector = second_vector - first_vector;
            
            perpendicular_clockwise_vector = line_vector.perpendicular_clockwise();
            perpendicular_counter_clockwise_vector = line_vector.perpendicular_counterclockwise();
            
            perpendicular_clockwise_vector.set_magnitude(relative_width);
            perpendicular_counter_clockwise_vector.set_magnitude(relative_width);
        }
        
        let points;
        {
            let side1_point1 = first_vector + perpendicular_counter_clockwise_vector;
            let side1_point2 = first_vector + perpendicular_clockwise_vector;
            
            let side2_point1 = second_vector + perpendicular_counter_clockwise_vector;
            let side2_point2 = second_vector + perpendicular_clockwise_vector;

            let point1 = TVector2::new_from_xy(side1_point1.x, side1_point1.y);
            let point2 = TVector2::new_from_xy(side1_point2.x, side1_point2.y);
            let point3 = TVector2::new_from_xy(side2_point1.x, side2_point1.y);
            let point4 = TVector2::new_from_xy(side2_point2.x, side2_point2.y);

            points = Vec::from([point1.clone(),point2,point3,point4, point1]);
        }

        self.enqueue_shape(points, brush, state);
    }
    
    
}