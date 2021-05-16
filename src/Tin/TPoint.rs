

pub struct TinPoint {
    pub x: f64,
    pub y: f64
}

impl TinPoint {
    pub fn new_from_coords(x: f64, y: f64) -> Self {
        Self { x: x, y: y }
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
    Deg(f64),
    Grad(f64),
    Rad(f64)
}
  
enum TinPoint {
    Cartesian(i64, i64),
    Polar(f64, Angle)
}
*/
