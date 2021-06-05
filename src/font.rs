
extern crate font_kit;
use font_kit::{family_name::FamilyName, properties::Properties, sources::fs::FsSource as FontSource, font::Font as Font};

use crate::{Double, Float, draw::text};


pub enum TinFontHorizontalAlignment {
    Left, Center, Right
}


pub enum TinFontVerticalAlignment {
    Bottom, Baseline, Center, Top
}


pub enum TinTextAlignment {
    Left, Right, Center, Justified, Natural
}


pub struct TinFont {
    font: Font,
    font_size: Float,
    pub line_height_multiple: Float,
    
    pub horizontal_alignment: TinFontHorizontalAlignment,
    pub vertical_alignment: TinFontVerticalAlignment,
    pub paragraph_alignment: TinTextAlignment,
    pub kerning: Float
}


impl TinFont {
    pub fn new(family_name: FamilyName, size: Float) -> Result<Self, ()> {
        let font = FontSource::new().select_best_match(
            &[family_name],
            &Properties::new())
            .unwrap()
            .load()
            .expect("Unable to initialize font");
            
        Ok(Self {
            font: font,
            font_size: size,
            line_height_multiple: 1.0,
            horizontal_alignment: TinFontHorizontalAlignment::Center,
            vertical_alignment: TinFontVerticalAlignment::Baseline,
            paragraph_alignment: TinTextAlignment::Left,
            kerning: 0.0
        })
    }

    /** Returns the amount of space needed to draw the text with the given font. */
    pub fn size(&self, _message: String) -> [f32;2] {
        todo!("Incomplete: Tin Font support not implemented");
    }

    pub fn draw(&self, message: &String, x: Double, y: Double) {
        text(message, &self, x, y)
    }
}
