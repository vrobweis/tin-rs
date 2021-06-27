use crate::{
    backends::{luminance::LuminanceBackend, ImageRenderer},
    image::TinImage,
    Double,
};

impl ImageRenderer for LuminanceBackend {
    fn image_with_size_and_resize(
        &mut self,
        image: &TinImage,
        center: crate::point::TinPoint,
        width: Double,
        height: Double,
        resize: bool,
        state: crate::context::DrawState,
    ) {
        todo!("LuminanceBackend::image_with_size_and_resize() not implemented");
    }
}
