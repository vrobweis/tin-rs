use crate::{backends::PathRenderer, point::TinPoint, context::get_tin, vertex::TinVertex};

use super::{LuminanceBackend, make_shape_from_vertex_vec, make_vertex};



impl PathRenderer for LuminanceBackend {
    
    fn path_begin(&mut self) {
        self.path_started = true
    }
    
    fn path_vertex(&mut self, at_point: &TinPoint) {
        self.path_vertices.push_back(make_vertex(at_point, &get_tin().get_fill_color()));
    }
    
    fn path_add_curve(&mut self, to: &TinPoint, control1: &TinPoint, control2: &TinPoint) {
        todo!("path_add_curve method in LuminanceBackend not supported yet");
        //cg.addCurve(to: to, control1: control1, control2: control2)
    }
    
    fn path_end(&mut self) {
        let mut verts: Vec<TinVertex> = Vec::new();

        for v in &mut self.path_vertices {
            verts.push(*v);
        }

        self.shape_queue.push_back( make_shape_from_vertex_vec(verts) ); 
    }
}