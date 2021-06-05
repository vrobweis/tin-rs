
#[allow(unused_imports)]
use assert_approx_eq::assert_approx_eq;
#[allow(unused_macros)]
macro_rules! assert_roughly_eq {
    ($x:expr, $y:expr) => {
        assert_approx_eq!($x , $y , 2.0e-3)
    }
}


mod color {
    use super::*;
    use crate::{Double, UInt, color::*};
    #[test]
    fn make_color_from_UInt() {
        let r: Double = 0.7;
        let g: Double = 0.6;
        let b: Double = 0.5;
        let a: Double = 0.4;
        let test_val = UInt::new_from_rgba(r, g, b, a);
        assert_roughly_eq!(r, test_val.get_red());
        assert_roughly_eq!(g, test_val.get_green());
        assert_roughly_eq!(b, test_val.get_blue());
        assert_roughly_eq!(a, test_val.get_alpha());
    }
}

mod calculation {
    use super::*;
    use crate::{Double, calculation::*};

    #[test]
    /// Test for constrain() function.
    fn test_constrain() {
        let v1: Double = constrain(150.0, 0.0, 100.0);
        let v2: Double = constrain(-150.0, 0.0, 100.0);
        assert_eq!(v1, 100.0);
        assert_eq!(v2, 0.0);
    }

    #[test]
    /// Test for linear interpolation, or lerp(), function.
    fn test_lerp() {
        let v1: Double = lerp(0.0, 100.0, 0.3);
        let v2: Double = lerp(150.0, 0.0, 0.5);
        assert_roughly_eq!(v1, 30.0);
        assert_roughly_eq!(v2, 75.0);
    }

    #[test]
    // Test for smoothstep() function.
    fn test_smoothstep() {
        let v: Double = smoothstep(0.0, 100.0, 0.7);
        assert_roughly_eq!( v , 78.4);
    }

    #[test]
    // Test for smootherstep() function.
    fn test_smootherstep() {
        let v: Double = smootherstep(0.0, 100.0, 0.7);
        assert_roughly_eq!( v , 83.69);
    }

    #[test]
    // Test for mag() function.
    fn test_mag() {
        let v: Double = mag(4.0, 4.0);
        assert_roughly_eq!( v , sqrt(32.0));
    }

    #[test]
    // Test for remap() function.
    fn test_remap() {
        let v: Double = remap(0.03, 0.0, 1.0, 100.0, 200.0);
        assert_roughly_eq!( v , 103.0);
    }

    #[test]
    // Test for norm() function.
    fn test_norm() {
        let v: Double = norm(4.0, 1.0, 7.0);
        assert_roughly_eq!( v , 0.5);
    }


use std::f64::consts::FRAC_PI_2;
    #[test]
    // Test for sq() function.
    fn test_sq() {
        let v: Double = sq(FRAC_PI_2);
        assert_roughly_eq!( v , FRAC_PI_2 * FRAC_PI_2);
    }

    #[test]
    // Test for sqrt() function.
    fn test_sqrt() {
        let v: Double = sqrt(FRAC_PI_2 * FRAC_PI_2);
        assert_roughly_eq!( v , FRAC_PI_2);
    }

    #[test]
    // Test for dist() function.
    fn test_dist() {
        let v: Double = dist(5.0,-3.0,3.0, 7.0);
        assert_roughly_eq!( v , sqrt(sq(3.0 - 5.0) + sq(7.0 - -3.0)));
    }
    
    #[test]
    // Test for to_degrees() function.
    fn test_to_degrees() {
        let v: Double = to_degrees(FRAC_PI_2);
        assert_roughly_eq!( v , 90.0);
    }

    #[test]
    // Test for to_radians() function.
    fn test_to_radians() {
        let v: Double = to_radians(90.0);
        assert_roughly_eq!( v , FRAC_PI_2);
    }
    
    
}



// TODO: Add test module for each module in library with utility methods that can be unit tested

mod vector2 {
    // TODO: Add Vector tests
}


