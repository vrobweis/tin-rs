use crate::{
    backends::{luminance::LuminanceBackend, LineRenderer},
    context::DrawState,
    point::TinPoint,
    vector2::TinVector2,
    Double,
};

impl LineRenderer for LuminanceBackend {
    /// TODO: Document this method.
    fn line(
        &mut self,
        point1: TinPoint,
        point2: TinPoint,
        width: Double,
        brush: crate::brush::TBrush,
        state: DrawState,
    ) {
        let relative_width = width / 2.0;
        let first_vector = TinVector2::from(point1);
        let second_vector = TinVector2::from(point2);

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

            let point1 = TinVector2::from_xy(side1_point1.x, side1_point1.y);
            let point2 = TinVector2::from_xy(side1_point2.x, side1_point2.y);
            let point3 = TinVector2::from_xy(side2_point1.x, side2_point1.y);
            let point4 = TinVector2::from_xy(side2_point2.x, side2_point2.y);

            points = Vec::from([point1.clone(), point2, point3, point4, point1]);
        }

        self.enqueue_shape(points, brush, state);
    }
}
