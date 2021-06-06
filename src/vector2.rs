use super::{Double, calculation::sqrt};


/**
 A structure to represent a two dimensional vector.
*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TVector2 {
    pub x: Double,
    pub y: Double
}



impl TVector2 {

    // MARK: - Initializers
    pub const fn new_from_xy(x: Double, y: Double) -> Self {
        Self {
            x: x,
            y: y
        }
    }

    pub fn new() -> Self {
        TVector2::new_from_xy(0.0, 0.0)
    }

    /// Produces vector with magnitude 1 from angle in radians
    pub fn new_from_angle(angle: Double) -> Self {
        TVector2::new_from_xy(angle.cos(), angle.sin())
    }

    pub fn get_x(&self) -> Double {
        self.x
    }
    pub fn get_y(&self) -> Double {
        self.y
    }

    pub fn set_x(&mut self, value: Double) {
        self.x = value
    }
    pub fn set_y(&mut self, value: Double) {
        self.y = value
    }

    pub fn get_magnitude(&self) -> Double {
        return sqrt((self.x * self.x) + (self.y * self.y))
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
    pub fn angle_between(v1: TVector2, v2: TVector2) -> Double {
        let mut n1 = v1;
        let mut n2 = v2;
        n1.normalize();
        n2.normalize();
        let dp = n1.dot(n2);
        let result = dp.acos();
        return result
    }
    
    

    pub fn perpendicular_clockwise(&self) -> TVector2 {
        return TVector2::new_from_xy(self.y, -self.x);
    }

    pub fn perpendicular_counterclockwise(&self) -> TVector2 {
        return TVector2::new_from_xy(-self.y, self.x);
    }
    
    // MARK: - Instance methods
    
    
    pub fn cross(&self, v: TVector2) -> Double {
        return self.x * v.y + self.y * v.x
    }
    
    pub fn distance(&self, v: TVector2) -> Double {
        let dx = self.x - v.x;
        let dy = self.y - v.y;
        return sqrt( dx * dx + dy * dy )
    }
    
    // TODO: why have this instance method?
    // is this used in a NOC example? If not, remove it.
    pub fn dot(&self, v: TVector2) -> Double {
        return self.x * v.x + self.y * v.y
    }
    
    pub fn heading(&self) -> Double {
        return self.y.atan2(self.x)
    }
    
    pub fn lerp(&mut self, v: TVector2, amount: Double ) {
        self.x = self.x + (v.x - self.x) * amount;
        self.y = self.y + (v.y - self.y) * amount;
    }
    
    pub fn limit(&mut self, mag: Double ) {
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
    
    pub fn rotate(&mut self, theta: Double ) {
        let temp = self.x;
        if theta % std::f64::consts::FRAC_2_PI == 0_f64 {return}
        self.x = self.x * theta.cos() - self.y * theta.sin();
        self.y = temp * theta.sin() + self.y * theta.cos();
    }
    
    
    // Squared magnitude
    pub fn magnitude_squared(&self) -> Double {
        return (self.x * self.x) + (self.y * self.y)
    }
    
}

use std::ops::{Add,AddAssign, Mul, MulAssign, Sub, SubAssign, Div, DivAssign, Neg};

impl Add for TVector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self { x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl Add<Double> for TVector2 {
    type Output = Self;

    fn add(self, rhs: Double) -> Self {
        Self { x: self.x + rhs, y: self.y + rhs}
    }
}

impl AddAssign for TVector2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl AddAssign<Double> for TVector2 {
    fn add_assign(&mut self, rhs: Double) {
        self.x = self.x + rhs;
        self.y = self.y + rhs;
    }
}

impl Div for TVector2 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self { x: self.x/rhs.x , y: self.y/rhs.y }
    }
}

impl Div<Double> for TVector2 {
    type Output = Self;

    fn div(self, rhs: Double) -> Self {
        Self { x: self.x/rhs , y: self.y/rhs }
    }
}

impl DivAssign for TVector2 {
    fn div_assign(&mut self, rhs: Self) {
        self.x = self.x / rhs.x;
        self.y = self.y / rhs.y;
    }
}

impl DivAssign<Double> for TVector2 {
    fn div_assign(&mut self, rhs: Double) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
    }
}

impl Mul for TVector2 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x , y: self.y * rhs.y
        }
    }
}

impl Mul<Double> for TVector2 {
    type Output = Self;
    fn mul(self, rhs: Double) -> Self {
        Self {
            x: self.x * rhs, y: self.y * rhs
        }
    }
}

impl MulAssign for TVector2 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x = self.x * rhs.x;
        self.y = self.y * rhs.y;
    }
}

impl MulAssign<Double> for TVector2 {
    fn mul_assign(&mut self, rhs: Double) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
    }
}

impl Sub for TVector2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {x: self.x - other.x, y: self.y - other.y}
    }
}

impl Sub<Double> for TVector2 {
    type Output = Self;
    fn sub(self, other: Double) -> Self {
        Self {x: self.x - other, y: self.y - other}
    }
}

impl SubAssign for TVector2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

impl SubAssign<Double> for TVector2 {
    fn sub_assign(&mut self, rhs: Double) {
        self.x = self.x - rhs;
        self.y = self.y - rhs;
    }
}

impl Neg for TVector2 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {x: -self.x, y: -self.y}
    }
}

impl ToString for TVector2 {
    fn to_string(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}

