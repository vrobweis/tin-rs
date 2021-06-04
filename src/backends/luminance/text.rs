use crate::{
    Double,
    backends::{TextRenderer, luminance::LuminanceBackend},
    font::TinFont
};


impl TextRenderer for LuminanceBackend {
    
    fn text(&mut self, message: &String, font: &TinFont, x: &Double, y: &Double) {
        todo!("text method in LuminanceBackend not supported yet");
    }

}

