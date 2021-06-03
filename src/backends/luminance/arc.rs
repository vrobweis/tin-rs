use crate::{Double, backends::ArcRenderer, context::{DrawState, DrawType}};

use super::LuminanceBackend;



impl ArcRenderer for LuminanceBackend {
    
    fn arc(&mut self, x: &Double, y: &Double, radius: &Double, start_angle: &Double, end_angle: &Double, brush: DrawType, state: DrawState) {
        eprintln!("LuminanceBackend::arc()");
        todo!("Arc render for LuminanceBackend not implemented");
    }
}