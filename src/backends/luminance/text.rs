use crate::{Double, backends::TextRenderer, font::TinFont};

use super::LuminanceBackend;



impl TextRenderer for LuminanceBackend {
    // MARK: - Text
    
    fn text(&mut self, message: &String, font: &TinFont, x: &Double, y: &Double) {
        todo!("text method in LuminanceBackend not supported yet");
        // Draw text at location with default size
    }
}