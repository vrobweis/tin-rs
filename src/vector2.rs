use crate::point::TPoint;

use super::{calculation::sqrt, Double};

/**
 A structure to represent a two dimensional vector.
*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TinVector2 {
    pub x: Double,
    pub y: Double,
}

impl TinVector2 {
    // MARK: - Initializers
    pub const fn from_xy(x: Double, y: Double) -> Self {
        Self { x, y }
    }

    pub const fn new() -> Self {
        TinVector2::from_xy(0.0, 0.0)
    }

    /// Produces vector with magnitude 1 from angle in radians
    pub fn from_angle(angle: Double) -> Self {
        TinVector2::from_xy(angle.cos(), angle.sin())
    }

    pub const fn get_x(&self) -> Double {
        self.x
    }
    pub const fn get_y(&self) -> Double {
        self.y
    }

    pub fn set_x(&mut self, value: Double) {
        self.x = value
    }
    pub fn set_y(&mut self, value: Double) {
        self.y = value
    }

    pub fn get_magnitude(&self) -> Double {
        return sqrt((self.x * self.x) + (self.y * self.y));
    }

    pub fn set_magnitude(&mut self, new_value: Double) {
        self.normalize();
        self.x = self.x * new_value;
        self.y = self.y * new_value;
    }

    // MARK: - Type methods

    /**
     Calculate the angle, in radians, between two vectors.

     - Parameter v1: The first vector.
     - Parameter v2: The second vector.

     - Returns: (Double) Result (in radians) angle between v1 and v2.
    */
    pub fn angle_between(v1: TinVector2, v2: TinVector2) -> Double {
        let mut n1 = v1;
        let mut n2 = v2;
        n1.normalize();
        n2.normalize();
        let dp = n1.dot(n2);
        let result = dp.acos();
        return result;
    }

    pub fn perpendicular_clockwise(&self) -> TinVector2 {
        return TinVector2::from_xy(self.y, -self.x);
    }

    pub fn perpendicular_counterclockwise(&self) -> TinVector2 {
        return TinVector2::from_xy(-self.y, self.x);
    }

    // MARK: - Instance methods

    pub fn cross(&self, v: TinVector2) -> Double {
        return self.x * v.y + self.y * v.x;
    }

    pub fn distance(&self, v: TinVector2) -> Double {
        let dx = self.x - v.x;
        let dy = self.y - v.y;
        return sqrt(dx * dx + dy * dy);
    }

    // TODO: why have this instance method?
    // is this used in a NOC example? If not, remove it.
    pub fn dot(&self, v: TinVector2) -> Double {
        return self.x * v.x + self.y * v.y;
    }

    pub fn heading(&self) -> Double {
        return self.y.atan2(self.x);
    }

    pub fn lerp(&mut self, v: TinVector2, amount: Double) {
        self.x = self.x + (v.x - self.x) * amount;
        self.y = self.y + (v.y - self.y) * amount;
    }

    pub fn limit(&mut self, mag: Double) {
        if self.magnitude_squared() > (mag * mag) {
            self.normalize();
            self.x = self.x * mag;
            self.y = self.y * mag;
        }
    }

    pub fn normalize(&mut self) {
        let mag = self.get_magnitude();
        if mag != 0.0 && mag != 1.0 {
            self.x = self.x / mag;
            self.y = self.y / mag;
        }
    }

    pub fn rotate(&mut self, theta: Double) {
        let temp = self.x;
        if theta % std::f64::consts::FRAC_2_PI == 0_f64 {
            return;
        }
        self.x = self.x * theta.cos() - self.y * theta.sin();
        self.y = temp * theta.sin() + self.y * theta.cos();
    }

    // Squared magnitude
    pub fn magnitude_squared(&self) -> Double {
        return (self.x * self.x) + (self.y * self.y);
    }
}

impl From<crate::point::TinPoint> for TinVector2 {
    fn from(p: crate::point::TinPoint) -> Self {
        Self {
            x: p.get_x(),
            y: p.get_y(),
        }
    }
}

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

impl Add for TinVector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Double> for TinVector2 {
    type Output = Self;

    fn add(self, rhs: Double) -> Self {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl AddAssign for TinVector2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl AddAssign<Double> for TinVector2 {
    fn add_assign(&mut self, rhs: Double) {
        self.x = self.x + rhs;
        self.y = self.y + rhs;
    }
}

impl Div for TinVector2 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl Div<Double> for TinVector2 {
    type Output = Self;

    fn div(self, rhs: Double) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DivAssign for TinVector2 {
    fn div_assign(&mut self, rhs: Self) {
        self.x = self.x / rhs.x;
        self.y = self.y / rhs.y;
    }
}

impl DivAssign<Double> for TinVector2 {
    fn div_assign(&mut self, rhs: Double) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
    }
}

impl Mul for TinVector2 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Mul<Double> for TinVector2 {
    type Output = Self;
    fn mul(self, rhs: Double) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign for TinVector2 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x = self.x * rhs.x;
        self.y = self.y * rhs.y;
    }
}

impl MulAssign<Double> for TinVector2 {
    fn mul_assign(&mut self, rhs: Double) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
    }
}

impl Sub for TinVector2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Sub<Double> for TinVector2 {
    type Output = Self;
    fn sub(self, other: Double) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
        }
    }
}

impl SubAssign for TinVector2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

impl SubAssign<Double> for TinVector2 {
    fn sub_assign(&mut self, rhs: Double) {
        self.x = self.x - rhs;
        self.y = self.y - rhs;
    }
}

impl Neg for TinVector2 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl ToString for TinVector2 {
    fn to_string(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}
