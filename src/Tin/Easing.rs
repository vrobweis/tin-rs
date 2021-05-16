//
//  Easing.rs
//  Tin
//  
//  Adapted by Vincent Weis on 5/9/2021
//_____________________________________
//  Original Swift version:
//  Easing.swift
//  Tin
//
//  Created by Loren Olson on 8/6/19.
//  Created at the School of Arts, Media and Engineering,
//  Herberger Institute for Design and the Arts,
//  Arizona State University.
//  Copyright (c) 2019 Arizona Board of Regents on behalf of Arizona State University
//
//  A few global useful functions for ease in/ease out calculations.
//



// Some specifics inspired by this reference.
// http://robertpenner.com/easing/
//
// Handy graphical reference to most of these
// https://easings.net/en

// MARK: - constants

#![allow(dead_code)]

use super::Calculation::remap;

type Double = f64;

const PI: Double = std::f64::consts::PI;

const PI_2: Double = PI / 2.0;


// For all the following easeIn, easeOut, and easeInOut functions...
// - Parameter value: an input Double in the range 0.0 to 1.0. Used to find
//                    an interpolated value for the range start to stop.
// - Parameter start: Beginning of the range for interpolation.
// - Parameter stop: End of the range for interpolation.


// MARK: - linear interpolation (reformulated version of lerp)
pub fn linear(value: Double, start: Double, stop: Double) -> Double {
    return remap(value, 0.0, 1.0, start, stop);
}


// MARK: - Quadratic ease in/out
// Modeled after the parabola y = x^2
pub fn easeInQuad(value: Double, start: Double, stop: Double) -> Double {
    let t = value * value;
    return remap(t, 0.0, 1.0, start, stop);
}

// Modeled after the parabola y = -x^2 + 2x
pub fn easeOutQuad(value: Double, start: Double, stop: Double) -> Double {
    let t = -(value * (value - 2.0));
    return remap( t,  0.0,  1.0, start, stop);
}

// Modeled after the piecewise quadratic
// y = (1/2)((2x)^2)             ; [0, 0.5)
// y = -(1/2)((2x-1)*(2x-3) - 1) ; [0.5, 1]
pub fn easeInOutQuad(value: Double, start: Double, stop: Double) -> Double {
    let t: Double;
    if value < 0.5 {
        t = 2.0 * value * value;
    }
    else {
        t = (-2.0 * value * value) + (4.0 * value) - 1.0;
    }
    return remap( t, 0.0, 1.0, start, stop);
}



// MARK: - Quartic ease in/out
// Modeled after the quartic x^4
pub fn easeInQuart(value: Double, start: Double, stop: Double) -> Double {
    let t = value * value * value * value;
    return remap( t, 0.0, 1.0, start, stop);
}

// Modeled after the quartic y = 1 - (x - 1)^4
pub fn easeOutQuart(value: Double, start: Double, stop: Double) -> Double {
    let f = value - 1.0;
    let t = f * f * f * (1.0 - value) + 1.0;
    return remap( t, 0.0, 1.0, start, stop);
}

// Modeled after the piecewise quartic
// y = (1/2)((2x)^4)        ; [0, 0.5)
// y = -(1/2)((2x-2)^4 - 2) ; [0.5, 1]
pub fn easeInOutQuart(value: Double, start: Double, stop: Double) -> Double {
    let t: Double;
    if value < 0.5 {
        t = 8.0 * value * value * value * value;
    }
    else {
        let f = value - 1.0;
        t = -8.0 * f * f * f * f + 1.0;
    }
    return remap( t, 0.0, 1.0, start, stop);
}



// MARK: - Quintic ease in/out
// Modeled after the quintic y = x^5
pub fn easeInQuint(value: Double, start: Double, stop: Double) -> Double {
    let t = value * value * value * value * value;
    return remap( t, 0.0, 1.0, start, stop);
}

// Modeled after the quintic y = (x - 1)^5 + 1
pub fn easeOutQuint(value: Double, start: Double, stop: Double) -> Double {
    let f = value - 1.0;
    let t = f * f * f * f * f + 1.0;
    return remap( t, 0.0, 1.0, start, stop);
}

// Modeled after the piecewise quintic
// y = (1/2)((2x)^5)       ; [0, 0.5)
// y = (1/2)((2x-2)^5 + 2) ; [0.5, 1]
pub fn easeInOutQuint(value: Double, start: Double, stop: Double) -> Double {
    let t: Double;
    if value < 0.5 {
        t = 16.0 * value * value * value * value * value;
    }
    else {
        let f = (2.0 * value) - 2.0;
        t = 0.5 * f * f * f * f * f + 1.0;
    }
    return remap( t, 0.0, 1.0, start, stop);
}



// MARK: - Overshooting cubic ease in/out
// Modeled after the overshooting cubic y = x^3-x*sin(x*PI)
pub fn easeInBack(value: Double, start: Double, stop: Double) -> Double {
    let t = value * value * value - value * (value * PI).sin();
    return remap( t, 0.0, 1.0, start, stop);
}

// Modeled after overshooting cubic y = 1-((1-x)^3-(1-x)*sin((1-x)*PI))
pub fn easeOutBack(value: Double, start: Double, stop: Double) -> Double {
    let f = 1.0 - value;
    let t = 1.0 - (f * f * f - f * (f * PI).sin());
    return remap( t, 0.0, 1.0, start, stop);
}

// Modeled after the piecewise overshooting cubic function:
// y = (1/2)*((2x)^3-(2x)*sin(2*x*PI))           ; [0, 0.5)
// y = (1/2)*(1-((1-x)^3-(1-x)*sin((1-x)*PI))+1) ; [0.5, 1]
pub fn easeInOutBack(value: Double, start: Double, stop: Double) -> Double {
    let t: Double;
    if value < 0.5 {
        let f = 2.0 * value;
        t = 0.5 * (f * f * f - f * (f * PI).sin());
    }
    else {
        let f = 1.0 - (2.0 * value - 1.0);
        t = 0.5 * (1.0 - (f * f * f - f * (f * PI).sin())) + 0.5;
    }
    return remap( t, 0.0, 1.0, start, stop);
}



// MARK: - Bouncing ease in/out
pub fn easeInBounce(value: Double, start: Double, stop: Double) -> Double {
    let t = 1.0 - easeOutBounce(1.0 - value, 0., 1.);
    return remap( t, 0.0, 1.0, start, stop);
}

pub fn easeOutBounce(value: Double, start: Double, stop: Double) -> Double {
    let t: Double;
    if value < 4.0 / 11.0 {
        t = (121.0 * value * value) / 16.0;
    }
    else if value < 8.0 / 11.0 {
        t = (363.0 / 40.0 * value * value) - (99.0 / 10.0 * value) + 17.0 / 5.0;
    }
    else if value < 9.0 / 10.0 {
        t = (4356.0 / 361.0 * value * value) - (35442.0 / 1805.0 * value) + 16061.0 / 1805.0;
    }
    else {
        t = (54.0 / 5.0 * value * value) - (513.0 / 25.0 * value) + 268.0 / 25.0;
    }
    return remap( t, 0.0, 1.0, start, stop)
}

pub fn easeInOutBounce(value: Double, start: Double, stop: Double) -> Double {
    let t: Double;
    if value < 0.5 {
        t = 0.5 * easeInBounce( value * 2.0, 0., 1.);
    }
    else {
        t = 0.5 * easeOutBounce( value * 2.0 - 1.0, 0., 1.) + 0.5;
    }
    return remap( t, 0.0, 1.0, start, stop)
}



// MARK: - Elastic ease in/out
// Modeled after the damped sine wave y = sin(13pi/2*x)*pow(2, 10 * (x - 1))
pub fn easeInElastic(value: Double, start: Double, stop: Double) -> Double {
    let t = (13.0 * PI_2 * value).sin() * (2.0 as Double).powf(10.0 * (value - 1.0));
    return remap( t, 0.0, 1.0, start, stop)
}

// Modeled after the damped sine wave y = sin(-13pi/2*(x + 1))*pow(2, -10x) + 1
pub fn easeOutElastic(value: Double, start: Double, stop: Double) -> Double {
    let t = (-13.0 * PI_2 * (value + 1.0)).sin() * (2.0 as Double).powf(-10.0 * value) + 1.0;
    return remap( t, 0.0, 1.0, start, stop)
}

// Modeled after the piecewise exponentially-damped sine wave:
// y = (1/2)*sin(13pi/2*(2*x))*pow(2, 10 * ((2*x) - 1))      ; [0,0.5)
// y = (1/2)*(sin(-13pi/2*((2x-1)+1))*pow(2,-10(2*x-1)) + 2) ; [0.5, 1]
pub fn easeInOutElastic(value: Double, start: Double, stop: Double) -> Double {
    let t: Double;
    if value < 0.5 {
        t = 0.5 * (13.0 * PI_2 * (2.0 * value)).sin() * (2.0 as Double).powf(10.0 * ((2.0 * value) - 1.0))
    }
    else {
        t = 0.5 * ((-13.0 * PI_2 * ((2.0 * value - 1.0) + 1.0)).sin() * (2.0 as Double).powf(-10.0 * (2.0 * value - 1.0)) + 2.0)
    }
    return remap( t, 0.0, 1.0, start, stop)
}