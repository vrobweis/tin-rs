use crate::{
    backends::{
        luminance::{make_shape_from_vertex_vec, LuminanceBackend},
        PathRenderer,
    },
    point::TPoint,
    vertex::TinVertex,
};

impl PathRenderer for LuminanceBackend {
    fn path_begin(&mut self) {
        self.path_started = true;
    }

    fn path_vertex(&mut self, at_point: &impl TPoint) {
        todo!("path_vertex method in LuminanceBackend not supported yet");
    }

    fn path_add_curve(&mut self, to: &impl TPoint, control1: &impl TPoint, control2: &impl TPoint) {
        todo!("path_add_curve method in LuminanceBackend not supported yet");
    }

    fn path_end(&mut self) {
        self.path_started = false;
        let mut verts: Vec<TinVertex> = Vec::new();
        for v in &mut self.path_vertices {
            verts.push(*v)
        }
        self.shape_queue
            .push_back(make_shape_from_vertex_vec(verts));
    }
}
