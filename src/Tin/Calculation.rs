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

type Double = f64;









/**
 Constrains a value to not exceed a minimum and maximum value.
 
 - Parameter value: (Double) Value to be constrained.
 - Parameter min: (Double) Lower limit, value will not be less than this.
 - Parameter max: (Double) Higher limit, value will not be greater than this.
 
 - Returns: (Double) Constrained value.
 
*/
#[allow(dead_code)]
pub fn constrain(value: Double, min: Double, max: Double) -> Double {
    if value < min {
        return min
    }
    else if value > max {
        return max
    }
    return value
}

/**
 Linearly interpolates between startValue and endVaue by t.
 
 When t = 0 returns startValue.
 When t = 1 return endValue.
 When t = 0.5 returns the midpoint of startValue and endValue.
 
 - Parameter startValue: (Double) Start value.
 - Parameter endValue: (Double) End value.
 - Parameter t: (Double) interpolation value.
 
 - Returns: (Double) interpolated value.
 
*/
#[allow(dead_code)]
pub fn lerp(startValue: Double, endValue: Double, t: Double) -> Double {
    return startValue + (endValue - startValue) * t;
}


/**
 Smoothstep (ease in/ease out) interpolates between startValue and endVaue by t.
 
 When t = 0 returns startValue.
 When t = 1 return endValue.
 When t = 0.5 returns the midpoint of startValue and endValue.
 
 - Parameter startValue: (Double) Start value.
 - Parameter endValue: (Double) End value.
 - Parameter t: (Double) Interpolation value.
 
 - Returns: (Double) Interpolated value.
 
*/
#[allow(dead_code)]
pub fn smoothstep(startValue: Double, endValue: Double, t: Double) -> Double {
    if t <= 0.0 {
        return startValue
    }
    else if t >= 1.0 {
        return endValue
    }
    else {
        let y = t * t * (3.0 - 2.0 * t);
        return startValue + ((endValue - startValue) * y)
    }
}


/**
 Calculates the magnitude of a (2d) vector.
 
 - Parameter x: (Double) First vector component.
 - Parameter y: (Double) Second vector component.
 
 - Returns: (Double) Magnitude (ie length).
*/
#[allow(dead_code)]
pub fn mag(x: Double, y: Double) -> Double {
    return sqrt(x * x + y * y)
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
#[allow(dead_code)]
pub fn remap(value: Double, start1: Double, stop1: Double, start2: Double, stop2: Double) -> Double {
    let result = start2 + (stop2 - start2) * ((value - start1) / (stop1 - start1));
    return result
}


/**
 Normalizes a number in another range into a value between 0 and 1.
 
 - Parameter value: (Double) Input value that will be normalized.
 - Parameter startValue: (Double) Lower bound of the value's current range.
 - Parameter endValue: (Double) Upper bound of the value's current range.
 
 - Returns: (Double) Normalized value.
*/
#[allow(dead_code)]
pub fn norm(value: Double, startValue: Double, endValue: Double) -> Double {
    return (value - startValue) / (endValue - startValue)
}

/**
 Calculate the square of a value.
 
 - Parameter value: (Double) Value to be squared.
 
 - Returns: (Double) Square of value.
*/
#[allow(dead_code)]
pub fn sq(value: Double) -> Double {
    return value * value
}


/**
 Calculate the square of a value.
 
 - Parameter value: (Double) Value to be squared.

 - Returns: (Double) Square of value.
*/
#[allow(dead_code)]
pub fn sqrt(value: Double) -> Double {
    return value.sqrt()
}


/**
 Calculate the distance between two 2D points.
 
 - Parameter x1: (Double) x value of point 1.
 - Parameter y1: (Double) y value of point 1.
 - Parameter x2: (Double) x value of point 2.
 - Parameter y2: (Double) y value of point 2.
 
 - Returns: (Double) distance.
*/
#[allow(dead_code)]
pub fn dist( x1: Double, y1: Double, x2: Double, y2: Double ) -> Double {
    return sqrt(sq(x2 - x1) + sq(y2 - y1))
}





#[allow(dead_code)]
const RADIANS_TO_DEGREES: Double = 180 as Double/std::f64::consts::PI;

#[allow(dead_code)]
const DEGREES_TO_RADIANS: Double = std::f64::consts::PI/180 as Double;

/**
 Convert from Radians to Degrees.
 
 - Parameter radians: (Double) Value in radians that will be converted.
 
 - Returns: (Double) Converted value in degrees.
*/
#[allow(dead_code)]
pub fn toDegrees(radians: Double) -> Double {
    return radians * RADIANS_TO_DEGREES
}


/**
 Convert from Degrees to Radians.
 
 - Parameter degrees: (Double) Value in degrees that will be converted.
 
 - Returns: (Double) Converted value in radians.
*/
#[allow(dead_code)]
pub fn toRadians(degrees: Double) -> Double {
    return degrees * DEGREES_TO_RADIANS
}


#[test]
fn MathTest() {
    let testDouble: Double= 80 as Double;
    assert_eq!(sqrt(sq(testDouble)), testDouble);

    const RIGHT_ANGLE_DEGREES: Double = 90.;
    // Linear interpolation test
    {
        const START_VALUE: Double = 20.;
        const END_VALUE: Double = 60.;
        const MIDPOINT: Double = ((END_VALUE - START_VALUE)/(2 as Double)) + START_VALUE;
        const INTERPOLATION_VAL: Double = 0.5;
        
        assert_eq!(lerp(START_VALUE, END_VALUE, INTERPOLATION_VAL), MIDPOINT);
    }
    

    // Test angle conversion methods
    {
        assert_eq!(toDegrees(toRadians(RIGHT_ANGLE_DEGREES)), RIGHT_ANGLE_DEGREES);
    }
    
}