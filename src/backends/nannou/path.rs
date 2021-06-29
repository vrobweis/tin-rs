use crate::{backends::{PathRenderer, nannou::NannouBackend}, point::TPoint};

impl PathRenderer for NannouBackend {
    // MARK: - Path stuff

    fn path_begin(&mut self) {
        todo!()
    }

    fn path_vertex(&mut self, at_point: &impl TPoint) {
        todo!();
    }

    fn path_add_curve(&mut self, to: &impl TPoint, control1: &impl TPoint, control2: &impl TPoint) {
        todo!("path_add_curve method in NannouBackend not supported yet");
        //cg.addCurve(to: to, control1: control1, control2: control2)
    }

    fn path_end(&mut self) {
        todo!("path_end method in NannouBackend not supported yet");
    }
}
