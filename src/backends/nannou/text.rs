use crate::{Double, backends::TextRenderer, font::TinFont};
use super::NannouBackend;

impl TextRenderer for NannouBackend {
    fn text(&mut self, message: &String, font: &TinFont, x: Double, y: Double) {
        todo!("text method in NannouBackend not supported yet");
        // Draw text at location with default size
    }
}