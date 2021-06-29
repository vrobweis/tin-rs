#[cfg(luminance_backend)]
pub(crate) mod luminance;
#[macro_use]
pub(crate) mod nannou;

use crate::{
    frame::TinFrame,
    scene::TScene,
    shapes::{
        ArcRenderer, EllipseRenderer, LineRenderer, PathRenderer, RectRenderer, TriangleRenderer,
    },
    Tin,
};

#[cfg(feature = "text")]
use crate::text::TextRenderer;

#[cfg(feature = "image")]
use crate::image::*;

const VS_STR: &str = include_str!("shaders/vertexshader.glsl");
const FS_STR: &str = include_str!("shaders/fragmentshader.glsl");

pub(crate) trait TBackend: TinRenderer {
    fn new() -> Self;
    fn run<S>(app: Tin<S>) -> Result<(), ()>
    where
        S: TScene + 'static;
}

pub(crate) trait TinRenderer
where
    Self: RectRenderer
        + TriangleRenderer
        + LineRenderer
        + ArcRenderer
        + EllipseRenderer
        + PathRenderer
        + StatefulRenderer
        + ImageRenderer
        + TextRenderer,
{
    /// Handle rendering setup.
    fn prepare(&mut self, _frame: TinFrame) {}

    // MARK: rendering cycle
    fn prepare_for_update(&mut self);
    fn did_finish_update(&mut self);
}

pub(crate) trait StatefulRenderer {
    fn push_state(&mut self);
    fn pop_state(&mut self);

    /*
        // enable "Restore from previous"
        // When enabled, this feature will cause the rendering to be saved in an image buffer.
        // After the double buffer swap, before the next render happens, the image saved
        // in the buffer is restored to the current frame buffer. This allows continuous draw effects.
        // Note, there is a time penalty for saving and restoring the image.
        pub fn enableRestoreFromPrevious(&mut self, ) {
            self.render.use_layer = true
        }


        pub fn disableRestoreFromPrevious(&mut self, ) {
            self.render.use_layer = false
        }
    */
}

#[cfg(not(feature = "image"))]
pub(crate) trait ImageRenderer {}

#[cfg(not(feature = "image"))]
impl<T> ImageRenderer for T where T: TinRenderer {}

#[cfg(not(feature = "text"))]
pub(crate) trait TextRenderer {}

#[cfg(not(feature = "text"))]
impl<T> TextRenderer for T where T: TinRenderer {}
