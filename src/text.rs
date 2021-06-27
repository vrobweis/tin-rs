extern crate font_kit;
use font_kit::{
    family_name::FamilyName, loaders::freetype::Font, properties::Properties,
    sources::fs::FsSource as FontSource,
};

use crate::{draw::text, point::TinPoint, Double, Float};

#[derive(Debug, Clone)]
pub enum TinFontHorizontalAlignment {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone)]
pub enum TinFontVerticalAlignment {
    Bottom,
    Baseline,
    Center,
    Top,
}

#[derive(Debug, Clone)]
pub enum TinTextAlignment {
    Left,
    Right,
    Center,
    Justified,
    Natural,
}

#[derive(Debug, Clone)]
pub struct TinFont {
    //pub(crate) font: Font, TODO: Implement generic means of selecting font
    pub(crate) font_size: Float,
    pub line_height_multiple: Float,

    pub horizontal_alignment: TinFontHorizontalAlignment,
    pub vertical_alignment: TinFontVerticalAlignment,
    pub paragraph_alignment: TinTextAlignment,
    pub kerning: Float,
}

impl TinFont {
    pub fn new(family_name: FamilyName, size: Float) -> Result<Self, ()> {
        let font = FontSource::new()
            .select_best_match(&[family_name], &Properties::new())
            .unwrap()
            .load()
            .expect("Unable to initialize font");

        Ok(Self {
            // font,
            font_size: size,
            line_height_multiple: 1.0,
            horizontal_alignment: TinFontHorizontalAlignment::Center,
            vertical_alignment: TinFontVerticalAlignment::Baseline,
            paragraph_alignment: TinTextAlignment::Left,
            kerning: 0.0,
        })
    }

    /** Returns the amount of space needed to draw the text with the given font. */
    pub fn size(&self, _message: String) -> [f32; 2] {
        todo!("Incomplete: Tin Font support not implemented");
    }

    pub fn draw(&self, message: &String, x: Double, y: Double) {
        text(message, &self, x, y)
    }
}

pub(crate) trait TextRenderer {
    fn text(
        &mut self,
        message: &String,
        font: &TinFont,
        center: TinPoint,
        state: crate::context::DrawState,
    ); // TODO: Implement TFont
}
