#[path = "backends/backends.rs"] pub(crate) mod backends;

pub mod calculation;
pub mod datetime;
pub mod draw;
pub mod easing;
pub mod event;
pub mod key;
pub mod noise;
pub mod color;
pub(crate) mod context;
pub mod controller;
pub mod font;
pub mod frame;
pub mod image;
pub mod point;
pub mod random; //TODO: Implement TRandom
pub mod scene;
pub mod shapes;
pub mod stopwatch;
pub mod vector2;
pub(crate) mod vertex;
pub mod view;


pub type Double = f64;
pub type Float = f32;
pub type Int = i32;
pub type UInt = u32;
pub type LongInt = i64;


pub use context::run;




#[macro_use] pub(crate) mod tests;