use crate::vector2::TVector2;
use super::assert_approx_eq;

// TODO: Add Vector tests

#[test]
fn test_math_with_tvector2() {
    const START: TVector2 = TVector2::new_from_xy(1.5,1.5);
    let mut test = START.clone();
    test += 0.5;
    test *= 2.0;
    test /= 2.0;
    test -= 0.5;
    assert_roughly_eq!(START.get_x(), test.get_x());
    assert_roughly_eq!(START.get_y(), test.get_y());
    assert_roughly_eq!(START.get_magnitude(), test.get_magnitude())
}