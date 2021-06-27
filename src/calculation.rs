//
//  Calculation.rs
//  Tin
//
//  Adapted by Vincent Weis on 5/9/2021
//_____________________________________
//  Original Swift version:
//  Created by Loren Olson on 10/22/16.
//  Created at the School of Arts, Media and Engineering,
//  Herberger Institute for Design and the Arts,
//  Arizona State University.
//  Copyright (c) 2017 Arizona Board of Regents on behalf of Arizona State University
//
//  A few global useful computation functions.

use crate::Double;

/**
 Constrains a value to not exceed a minimum and maximum value.

 - Parameter value: (Double) Value to be constrained.
 - Parameter min: (Double) Lower limit, value will not be less than this.
 - Parameter max: (Double) Higher limit, value will not be greater than this.

 - Returns: (Double) Constrained value.
*/
pub fn constrain(value: Double, min: Double, max: Double) -> Double {
    value.clamp(min, max)
}

/**
 Linearly interpolates between start_value and end_value by t.

 When t = 0 returns start_value.
 When t = 1 return end_value.
 When t = 0.5 returns the midpoint of start_value and end_value.

 - Parameter start_value: (Double) Start value.
 - Parameter end_value: (Double) End value.
 - Parameter t: (Double) interpolation value.

 - Returns: (Double) interpolated value.

*/

pub fn lerp(start_value: Double, end_value: Double, t: Double) -> Double {
    let range = end_value - start_value;
    let mapped_without_offset = range * t;
    let offset = start_value;
    return mapped_without_offset + offset;
}

/**
 Smoothstep (ease in/ease out) interpolates between start_value and endVaue by t.

 When t = 0 returns start_value.
 When t = 1 return end_value.
 When t = 0.5 returns the midpoint of start_value and end_value.

 - Parameter start_value: (Double) Start value.
 - Parameter end_value: (Double) End value.
 - Parameter t: (Double) Interpolation value.

 - Returns: (Double) Interpolated value.

*/
pub fn smoothstep(start_value: Double, end_value: Double, t: Double) -> Double {
    if t <= 0.0 {
        return start_value;
    } else if t >= 1.0 {
        return end_value;
    } else {
        let y = t.powi(2) * (3.0 - 2.0 * t);
        let offset = start_value;
        let range = end_value - start_value;
        return offset + (range * y);
    }
}

/**
 Smootherstep (ease in/ease out) interpolates between start_value and endVaue by t.
 (As suggested by Ken Perlin.)

 When t = 0 returns start_value.
 When t = 1 return end_value.
 When t = 0.5 returns the midpoint of start_value and end_value.

 - Parameter start_value: (Double) Start value.
 - Parameter end_value: (Double) End value.
 - Parameter t: (Double) Interpolation value.

 - Returns: (Double) Interpolated value.

*/
pub fn smootherstep(start_value: Double, end_value: Double, t: Double) -> Double {
    if t <= 0.0 {
        return start_value;
    } else if t >= 1.0 {
        return end_value;
    } else {
        let y = ((6. * t.powi(2)) - (15. * t) + 10.) * t.powi(3);
        let offset = start_value;
        let range = end_value - start_value;
        return offset + (range * y);
    }
}

/**
 Calculates the magnitude of a (2d) vector.

 - Parameter x: (Double) First vector component.
 - Parameter y: (Double) Second vector component.

 - Returns: (Double) Magnitude (ie length).
*/

pub fn mag(x: Double, y: Double) -> Double {
    return sqrt(x * x + y * y);
}

/**
 Remap a number from one range to another.
 Numbers outside the input range are not clamped,
 since out of range can be useful.
 Example: If the input range is 0 to 1, and
 output range is 0 to 100. Given the value 0.25,
 the result 25.0 is returned.

 - Parameter value: (Double) Input value to be re-mapped.
 - Parameter start1: (Double) Lower bound of the input range.
 - Parameter stop1: (Double) Upper bound of the input range.
 - Parameter start2: (Double) Lower bound of the output range.
 - Parameter stop2: (Double) Upper bound of the output range.

 - Returns: (Double) re-mapped value.
*/
pub fn remap(
    value: Double,
    start1: Double,
    stop1: Double,
    start2: Double,
    stop2: Double,
) -> Double {
    let first_range_distance = stop1 - start1;
    let second_range_distance = stop2 - start2;
    let value_without_offset = value - start1;
    let result_offset = start2;
    if first_range_distance == 0.0 || second_range_distance == 0.0 {
        panic!("The remap() function requires that the input and output range must be greater than zero.")
    }
    let result =
        result_offset + second_range_distance * (value_without_offset / first_range_distance);
    return result;
}

/**
 Normalizes a number in another range into a value between 0 and 1.

 - Parameter value: (Double) Input value that will be normalized.
 - Parameter start_value: (Double) Lower bound of the value's current range.
 - Parameter end_value: (Double) Upper bound of the value's current range.

 - Returns: (Double) Normalized value.
*/
pub fn norm(value: Double, start_value: Double, end_value: Double) -> Double {
    let range = end_value - start_value;
    let offset_value = value - start_value;
    return offset_value / range;
}

/**
 Calculate the square of a value.

 - Parameter value: (Double) Value to be squared.

 - Returns: (Double) Square of value.
*/

pub fn sq(value: Double) -> Double {
    return value.powi(2);
}

/**
 Calculate the square of a value.

 - Parameter value: (Double) Value to be squared.

 - Returns: (Double) Square of value.
*/

pub fn sqrt(value: Double) -> Double {
    return value.sqrt();
}

/**
 Calculate the distance between two 2D points.

 - Parameter x1: (Double) x value of point 1.
 - Parameter y1: (Double) y value of point 1.
 - Parameter x2: (Double) x value of point 2.
 - Parameter y2: (Double) y value of point 2.

 - Returns: (Double) distance.
*/

pub fn dist(x1: Double, y1: Double, x2: Double, y2: Double) -> Double {
    return sqrt(sq(x2 - x1) + sq(y2 - y1));
}

/**
 Convert from Radians to Degrees.

 - Parameter radians: (Double) Value in radians that will be converted.

 - Returns: (Double) Converted value in degrees.
*/

pub fn to_degrees(radians: Double) -> Double {
    return radians.to_degrees();
}

/**
 Convert from Degrees to Radians.

 - Parameter degrees: (Double) Value in degrees that will be converted.

 - Returns: (Double) Converted value in radians.
*/

pub fn to_radians(degrees: Double) -> Double {
    return degrees.to_radians();
}
