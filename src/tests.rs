
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
    use crate::color::*;
    #[test]
    fn make_color_from_u32() {
        let r: f64 = 0.7;
        let g: f64 = 0.6;
        let b: f64 = 0.5;
        let a: f64 = 0.4;
        let test_val = u32::new_from_rgba(r, g, b, a);
        assert_roughly_eq!(r, test_val.get_red());
        assert_roughly_eq!(g, test_val.get_green());
        assert_roughly_eq!(b, test_val.get_blue());
        assert_roughly_eq!(a, test_val.get_alpha());
    }
}

mod calculation {
    use super::*;
    use crate::calculation::*;

    #[test]
    /// Test for constrain() function.
    fn test_constrain() {
        let v1: f64 = constrain(150.0, 0.0, 100.0);
        let v2: f64 = constrain(-150.0, 0.0, 100.0);
        assert_eq!(v1, 100.0);
        assert_eq!(v2, 0.0);
    }

    #[test]
    /// Test for linear interpolation, or lerp(), function.
    fn test_lerp() {
        let v1: f64 = lerp(0.0, 100.0, 0.3);
        let v2: f64 = lerp(150.0, 0.0, 0.5);
        assert_roughly_eq!(v1, 30.0);
        assert_roughly_eq!(v2, 75.0);
    }

    #[test]
    // Test for smoothstep() function.
    fn test_smoothstep() {
        let v: f64 = smoothstep(0.0, 100.0, 0.7);
        assert_roughly_eq!( v , 78.4);
    }

    #[test]
    // Test for smootherstep() function.
    fn test_smootherstep() {
        let v: f64 = smootherstep(0.0, 100.0, 0.7);
        assert_roughly_eq!( v , 83.69);
    }

    #[test]
    // Test for mag() function.
    fn test_mag() {
        let v: f64 = mag(4.0, 2.0);
        assert_roughly_eq!( v , 6.0);
    }

    #[test]
    // Test for remap() function.
    fn test_remap() {
        let v: f64 = remap(0.03, 0.0, 1.0, 100.0, 200.0);
        assert_roughly_eq!( v , 103.0);
    }

    #[test]
    // Test for norm() function.
    fn test_norm() {
        let v: f64 = norm(4.0, 1.0, 7.0);
        assert_roughly_eq!( v , 0.5);
    }

    #[test]
    // Test for sq() function.
    fn test_sq() {
        let v: f64 = sq(std::f64::consts::FRAC_PI_2);
        assert_roughly_eq!( v , std::f64::consts::FRAC_PI_2 * std::f64::consts::FRAC_PI_2);
    }

    #[test]
    // Test for sqrt() function.
    fn test_sqrt() {
        let v: f64 = sqrt(std::f64::consts::FRAC_PI_2 * std::f64::consts::FRAC_PI_2);
        assert_roughly_eq!( v , std::f64::consts::FRAC_PI_2);
    }

    #[test]
    // Test for dist() function.
    fn test_dist() {
        let v: f64 = dist(5.0,-3.0,3.0, 7.0);
        assert_roughly_eq!( v , sqrt(sq(3.0 - 5.0) + sq(7.0 - -3.0)));
    }
    
    #[test]
    // Test for to_degrees() function.
    fn test_to_degrees() {
        let v: f64 = to_degrees(std::f64::consts::FRAC_PI_2);
        assert_roughly_eq!( v , 90.0);
    }

    #[test]
    // Test for to_radians() function.
    fn test_to_radians() {
        let v: f64 = to_radians(90.0);
        assert_roughly_eq!( v , std::f64::consts::FRAC_PI_2);
    }
    
    
}



// TODO: Add test module for each module in library with utility methods that can be unit tested

mod vector2 {
    // TODO: Add Vector tests
}


