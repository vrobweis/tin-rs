use crate::{Double, backends::ArcRenderer};

use super::NannouBackend;



impl ArcRenderer for NannouBackend {
    fn arc(&mut self, x: Double, y: Double, radius: Double, start_angle: Double, end_angle: Double) {
        eprintln!("NannouBackend::arc()");
        todo!("Arc render for NannouBackend not implemented");
    }
}