//
//  TFont.swift
//  Tin
//
//  Created by Loren Olson on 1/4/17.
//  Created at the School of Arts, Media and Engineering,
//  Herberger Institute for Design and the Arts,
//  Arizona State University.
//  Copyright (c) 2017 Arizona Board of Regents on behalf of Arizona State University
//

extern crate font_kit;


use font_kit::canvas::{Canvas, Format, RasterizationOptions};
use font_kit::family_name::FamilyName;
use font_kit::hinting::HintingOptions;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use font_kit::loaders::directwrite::Font as Font;

#[allow(dead_code)]
pub enum TinFontHorizontalAlignment {
    Left, Center, Right
}

#[allow(dead_code)]
pub enum TinFontVerticalAlignment {
    Bottom, Baseline, Center, Top
}

#[allow(dead_code)]
pub enum TinTextAlignment {
    Left, Right, Center, Justified, Natural
}

#[allow(dead_code)]
pub struct TinFont {
    font: Font,
    font_size: f32,
    pub lineHeightMultiple: f32,
    
    pub horizontalAlignment: TinFontHorizontalAlignment,
    pub verticalAlignment: TinFontVerticalAlignment,
    pub paragraphAlignment: TinTextAlignment,
    pub kerning: f32
}

#[allow(dead_code)]
impl TinFont {
    pub fn init(familyName: FamilyName, size: f32) -> Result<Self, ()> {
        todo!("Incomplete: Tin Font support not implemented");
        let font = SystemSource::new().select_best_match(&[familyName],
            &Properties::new()).unwrap().load()
            .expect("Unable to initialize font");
            
        //debug("font \(fontName) loaded.")
        Ok(Self {
            font: font,
            font_size: size,
            lineHeightMultiple: 1.0,
            horizontalAlignment: TinFontHorizontalAlignment::Center,
            verticalAlignment: TinFontVerticalAlignment::Baseline,
            paragraphAlignment: TinTextAlignment::Left,
            kerning: 0.0
        })
    }

    /** Returns the amount of space needed to draw the text with the given font. */
    pub fn size(&self, message: String) -> [f32;2] {
        todo!("Incomplete: Tin Font support not implemented");
        //let attributes = makeAttributes();
        //let str: NSAttributedString = NSAttributedString(string: message, attributes: attributes);
        //return str.size()
    }
}
