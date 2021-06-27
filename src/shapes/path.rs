use crate::point::TPoint;

pub(crate) trait PathRenderer {
    fn path_begin(&mut self);
    fn path_vertex(&mut self, at_point: &impl TPoint);
    fn path_add_curve(&mut self, to: &impl TPoint, control1: &impl TPoint, control2: &impl TPoint);
    fn path_end(&mut self);
}
