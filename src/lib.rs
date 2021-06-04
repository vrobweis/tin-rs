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
pub type Long = i64;
pub type Int = i32;
pub type Short = i16;
pub type ULong = u64;
pub type UInt = u32;
pub type UShort = u16;



pub use context::run;


pub(crate) type CurrentBackend = backends::luminance::LuminanceBackend;


#[macro_use] pub(crate) mod tests;