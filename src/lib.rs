#[deny(missing_docs)]
#[path = "backends/backends.rs"]
pub(crate) mod backends;

pub mod calculation;
#[cfg(feature = "time")]
pub mod datetime;
pub mod draw;
pub mod easing;

pub mod event;
pub use event::TinEvent;

pub mod key;
pub use key::TinKey;

pub mod noise;

pub(crate) mod brush;

pub mod color;
pub use color::{TColor, TinColor};

pub(crate) mod context;
pub mod frame;
#[cfg(feature = "image")]
pub mod image;
pub mod point;
#[cfg(feature = "random")]
pub mod random; //TODO: Implement TRandom
pub mod scene;
pub mod shapes;
pub mod stopwatch;
#[cfg(feature = "text")]
pub mod text;

pub mod vector2;
pub use vector2::TinVector2;


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

pub(crate) mod private {
    pub trait Sealed {}
}

pub(crate) mod app;
pub use app::*;

pub(crate) type CurrentBackend = backends::nannou::NannouBackend;

#[macro_use]
#[cfg(test)]
pub(crate) mod tests;
