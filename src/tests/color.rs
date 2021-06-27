use super::assert_approx_eq;
use crate::{color::*, Double, UInt};
#[test]
fn make_color_from_unsigned_int() {
    let r: Double = 0.7;
    let g: Double = 0.6;
    let b: Double = 0.5;
    let a: Double = 0.4;
    let test_val = UInt::from_rgba(r, g, b, a);
    assert_roughly_eq!(r, test_val.get_red());
    assert_roughly_eq!(g, test_val.get_green());
    assert_roughly_eq!(b, test_val.get_blue());
    assert_roughly_eq!(a, test_val.get_alpha());
}
