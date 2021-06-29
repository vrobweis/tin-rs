use crate::backends::{ImageRenderer, nannou::NannouBackend};

impl ImageRenderer for NannouBackend {
    fn image_with_size_and_resize(
        &mut self,
        image: &crate::image::TinImage,
        center: crate::point::TinPoint,
        width: crate::Double,
        height: crate::Double,
        resize: bool,
        state: crate::context::DrawState,
    ) {
        let draw = self
            .get_draw()
            .scale(state.scale as f32)
            .translate(nannou::prelude::Vector3::<f32>::new(
                state.translation.0 as f32,
                state.translation.1 as f32,
                0.0,
            ))
            .rotate(state.rotation as f32);
        todo!()
    }
}
