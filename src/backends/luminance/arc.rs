use crate::{Double, backends::ArcRenderer, context::{DrawState, TBrush}};

use super::LuminanceBackend;



impl ArcRenderer for LuminanceBackend {
    
    fn arc(&mut self, x: &Double, y: &Double, radius: &Double, start_angle: &Double, end_angle: &Double, brush: TBrush, state: DrawState) {
        eprintln!("LuminanceBackend::arc()");
        todo!("Arc render for LuminanceBackend not implemented");
    }
}