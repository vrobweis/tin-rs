use crate::{backends::PathRenderer, context::get_tin, point::{TPoint, TinPoint}, vertex::TinVertex};

use super::{NannouBackend, make_shape_from_vertex_vec, make_vertex};



impl PathRenderer for NannouBackend {
    // MARK: - Path stuff
    
    fn path_begin(&mut self) {
        self.path_started = true
    }
    
    fn path_vertex(&mut self, at_point: &impl TPoint) {
        self.path_vertices.push_back(make_vertex(at_point, &get_tin().get_fill_color()));
    }
    
    fn path_add_curve(&mut self, to: &impl TPoint, control1: &impl TPoint, control2: &impl TPoint) {
        todo!("path_add_curve method in NannouBackend not supported yet");
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