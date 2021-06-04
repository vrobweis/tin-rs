use crate::{
    backends::{
        luminance::{LuminanceBackend, make_shape_from_vertex_vec, make_vertex}, 
        PathRenderer
    }, 
    point::TinPoint, 
    context::get_tin, 
    vertex::TinVertex
};


impl PathRenderer for LuminanceBackend {
    
    fn path_begin(&mut self) {
        self.path_started = true;
    }
    
    fn path_vertex(&mut self, at_point: &TinPoint) {
        todo!("path_vertex method in LuminanceBackend not supported yet");
    }
    
    fn path_add_curve(&mut self, to: &TinPoint, control1: &TinPoint, control2: &TinPoint) {
        todo!("path_add_curve method in LuminanceBackend not supported yet");
    }
    
    fn path_end(&mut self) {
        self.path_started = false;
        let mut verts: Vec<TinVertex> = Vec::new();
        for v in &mut self.path_vertices { verts.push(*v) }
        self.shape_queue.push_back( make_shape_from_vertex_vec(verts) ); 
    }

}

