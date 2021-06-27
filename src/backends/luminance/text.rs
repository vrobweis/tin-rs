use crate::{
    backends::luminance::LuminanceBackend,
    point::TinPoint,
    text::{TextRenderer, TinFont},
};

impl TextRenderer for LuminanceBackend {
    fn text(
        &mut self,
        message: &String,
        font: &TinFont,
        center: TinPoint,
        state: crate::context::DrawState,
    ) {
        todo!("text method in LuminanceBackend not supported yet");
    }
}
