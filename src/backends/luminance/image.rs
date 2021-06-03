use crate::{Double, backends::ImageRenderer, image::TinImage};

use super::LuminanceBackend;



impl ImageRenderer for LuminanceBackend{
    fn image_with_size_and_resize(&mut self, image: &TinImage, x: &Double, y: &Double, width: &Double, height: &Double, resize: bool) {
        todo!("LuminanceBackend::image_with_size_and_resize() not implemented");
    }
}