
#[allow(unused_imports)]
use assert_approx_eq::assert_approx_eq;
#[allow(unused_macros)]
macro_rules! assert_roughly_eq {
    ($x:expr, $y:expr) => {
        assert_approx_eq!($x , $y , 2.0e-3)
    }
}


mod color;
mod calculation;
mod vector2;


// TODO: Add test module for each module in library with utility methods that can be unit tested




