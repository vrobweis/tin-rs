use crate::{
    Double, 
    backends::{luminance::LuminanceBackend, ArcRenderer}, 
    context::{DrawState, TBrush}
};


impl ArcRenderer for LuminanceBackend {
    
    fn arc(&mut self, x: Double, y: Double, radius: Double, start_angle: Double, end_angle: Double, brush: TBrush, state: DrawState) {
        todo!("Arc render for LuminanceBackend not implemented");
    }
    
}

