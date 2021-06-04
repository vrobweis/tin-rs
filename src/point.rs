
use crate::Double;
pub struct TinPoint {
    pub x: Double,
    pub y: Double
}

impl TinPoint {
    pub fn new_from_coords(x: Double, y: Double) -> Self {
        Self { x: x, y: y }
    }
}

impl Default for TinPoint {
    fn default() -> Self {
        TinPoint::new_from_coords(0.0, 0.0)
    }
}

impl Clone for TinPoint {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y
        }
    }
}


/*
TODO: Implement new point and angle types for Tin

enum TinAngle {
    Deg(Double),
    Grad(Double),
    Rad(Double)
}
  
enum TinPoint {
    Cartesian(i64, i64),
    Polar(Double, Angle)
}
*/
