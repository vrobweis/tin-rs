//
//  TVector2.swift
//  Tin
//
//  Created by Loren Olson on 10/3/16.
//  Created at the School of Arts, Media and Engineering,
//  Herberger Institute for Design and the Arts,
//  Arizona State University.
//  Copyright (c) 2017 Arizona Board of Regents on behalf of Arizona State University
//

use std::ops::{Add, Sub};
use super::{Double, Float};
use super::{Calculation::sqrt};
use super::Calculation;


// TODO: Implement unit tests for TVector2

/**
 A structure to represent a two dimensional vector.
*/
#[derive(Debug, Clone, Copy)]
pub struct TVector2 {
    pub x: Double,
    pub y: Double

}

#[allow(dead_code)]

impl TVector2 {

    // MARK: - Initializers
    pub fn new_from_xy(x: Double, y: Double) -> Self {
        Self {
            x: x,
            y: y
        }
    }

    pub fn new() -> Self {
        TVector2::new_from_xy(0.0, 0.0)
    }

    pub fn new_from_angle(angle: Double) -> Self {
        TVector2::new_from_xy(angle.cos(), angle.sin())
    }

    pub fn get_magnitude(&self) -> Double {
        return sqrt((self.x * self.x) + (self.y * self.y))
    }

    pub fn set_magnitude(&mut self, newValue: Double) {
        self.normalize();
        self.x = self.x * newValue;
        self.y = self.y * newValue;
    }
    
    
    
    
    // MARK: - Type methods
    
    /**
     Calculate the angle, in radians, between two vectors.
     
     - Parameter v1: The first vector.
     - Parameter v2: The second vector.
     
     - Returns: (Double) Result (in radians) angle between v1 and v2.
    */
    pub fn angleBetween(v1: TVector2, v2: TVector2) -> Double {
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
        if self.magSq() > (mag * mag) {
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
        self.x = self.x * theta.cos() - self.y * theta.sin();
        self.y = temp * theta.sin() + self.y * theta.cos();
    }
    
    
    // Squared magnitude
    pub fn magSq(&self) -> Double {
        return (self.x * self.x) + (self.y * self.y)
    }
    
}

impl std::ops::Add for TVector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self { x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl std::ops::AddAssign for TVector2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl std::ops::Div for TVector2 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self { x: self.x/rhs.x , y: self.y/rhs.y }
    }
}

impl std::ops::DivAssign for TVector2 {

    fn div_assign(&mut self, rhs: Self) {
        self.x = self.x / rhs.x;
        self.y = self.y / rhs.y;
    }
}

impl std::ops::Mul for TVector2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x , y: self.y * rhs.y
        }
    }
}

impl std::ops::MulAssign for TVector2 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x = self.x * rhs.x;
        self.y = self.y * rhs.y;
    }
}

impl std::ops::Sub for TVector2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {x: self.x - other.x, y: self.y - other.y}
    }
}

impl std::ops::SubAssign for TVector2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

impl std::ops::Neg for TVector2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {x: -self.x, y: -self.y}
    }
}



// MARK: - Operators
/*
public prefix func +(left: TVector2) -> TVector2 {
    return left
}

public func *(left: TVector2, right: Double) -> TVector2 {
    let x = left.x * right
    let y = left.y * right
    return TVector2(x: x, y: y)
}

public func *(left: Double, right: TVector2) -> TVector2 {
    let x = left * right.x
    let y = left * right.y
    return TVector2(x: x, y: y)
}

public func /(left: TVector2, right: Double) -> TVector2 {
    let x = left.x / right
    let y = left.y / right
    return TVector2(x: x, y: y)
}

public func *=(left: inout TVector2, right: Double) {
    left = left * right
}

public func /=(left: inout TVector2, right: Double) {
    left = left / right
}
*/