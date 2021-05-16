

pub struct TinPoint {
    pub x: f32,
    pub y: f32
}

impl TinPoint {
    pub fn new_from_coords(x: f32, y: f32) -> Self {
        Self { x: x, y: y }
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
