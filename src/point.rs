use crate::{
    calculation::{sq, sqrt},
    private::Sealed,
    Double, TinVector2,
};
pub trait TPoint: Sealed {
    fn from_coords(x: Double, y: Double) -> Self;

    fn set_x(&mut self, x: Double);
    fn set_y(&mut self, y: Double);

    fn get_x(&self) -> Double;
    fn get_y(&self) -> Double;

    fn translate(&mut self, dx: Double, dy: Double) {
        self.set_x(self.get_x() + dx);
        self.set_y(self.get_y() + dy);
    }

    fn rotate_around_origin(&mut self, theta: Double) {
        let mut v = TinVector2::from_xy(self.get_x(), self.get_y());
        v.rotate(theta);
        self.set_x(v.x);
        self.set_y(v.y);
    }

    fn set_polar_angle(&mut self, theta: Double) {
        self.set_x(self.get_polar_radius() * theta.cos());
        self.set_y(self.get_polar_radius() * theta.sin());
    }
    fn set_polar_radius(&mut self, r: Double) {
        self.set_x(r * self.get_polar_angle().cos());
        self.set_y(r * self.get_polar_angle().sin());
    }

    fn get_polar_angle(&self) -> Double {
        (self.get_y() / self.get_x()).atan()
    }
    fn get_polar_radius(&self) -> Double {
        sqrt(sq(self.get_x()) + sq(self.get_y()))
    }
}

#[derive(Debug, Clone)]
pub struct TinPoint {
    x: Double,
    y: Double,
}

impl TPoint for TinPoint {
    fn from_coords(x: Double, y: Double) -> Self {
        Self { x: x, y: y }
    }

    fn set_x(&mut self, x: Double) {
        self.x = x
    }

    fn set_y(&mut self, y: Double) {
        self.y = y
    }

    fn get_x(&self) -> Double {
        self.x
    }

    fn get_y(&self) -> Double {
        self.y
    }
}
impl Sealed for TinPoint {}

impl Default for TinPoint {
    fn default() -> Self {
        TinPoint::from_coords(0.0, 0.0)
    }
}

impl TPoint for (Double, Double) {
    fn from_coords(x: Double, y: Double) -> Self {
        (x, y)
    }

    fn set_x(&mut self, x: Double) {
        self.0 = x
    }

    fn set_y(&mut self, y: Double) {
        self.1 = y
    }

    fn get_x(&self) -> Double {
        self.0
    }

    fn get_y(&self) -> Double {
        self.1
    }
}
impl Sealed for (Double, Double) {}

impl TPoint for [Double; 2] {
    fn from_coords(x: Double, y: Double) -> Self {
        [x, y]
    }

    fn set_x(&mut self, x: Double) {
        self[0] = x
    }

    fn set_y(&mut self, y: Double) {
        self[1] = y
    }

    fn get_x(&self) -> Double {
        self[0]
    }

    fn get_y(&self) -> Double {
        self[1]
    }
}
impl Sealed for [Double; 2] {}
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
