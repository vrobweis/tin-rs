//
//  TinColor.rs
//  Tin
//  
//  Adapted by Vincent Weis on 5/9/2021
//_____________________________________
//  Original Swift version:
//  TinColor.swift
//  Tin
//
//  Created by Loren Olson on 3/10/17.
//  Copyright © 2017 ASU. All rights reserved.
//

use super::{Calculation::{constrain}, fillColor_from_RGBA, strokeColor_from_RGBA, Double, Float};

pub struct TPixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8
}

#[allow(dead_code)]
impl TPixel {
    pub fn new_from_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red: red,
            green: green,
            blue: blue,
            alpha: alpha,
        }
    }

    pub fn new_from_rgb(red: u8, green: u8, blue: u8) -> Self {
        TPixel::new_from_rgba(red,green,blue,255 as u8)
    }
    
    pub fn new_from_color(color: TinColor) -> Self {
        let r: u8 = constrain(color.red * 255.0, 0.0, 255.0) as u8;
        let g: u8 = constrain(color.green * 255.0, 0.0, 255.0) as u8;
        let b: u8 = constrain(color.blue * 255.0, 0.0, 255.0) as u8;
        let a: u8 = constrain(color.alpha * 255.0, 0.0, 255.0) as u8;
        TPixel::new_from_rgba( r, g, b, a)
    }
}

#[derive(Clone, Copy)]
pub struct TinColor {
    pub red: Double,
    pub green: Double,
    pub blue: Double,
    pub alpha: Double
}



pub const DEFAULT_COLOR_FILL: TinColor = TinColor {
    red: 0.7,
    green: 0.7,
    blue: 0.7,
    alpha: 1.0
};
pub const DEFAULT_COLOR_STROKE: TinColor = TinColor {
    red: 0.1,
    green: 0.1,
    blue: 0.1,
    alpha: 1.0
};
pub const DEFAULT_COLOR_BACKGROUND: TinColor = TinColor {
    red: 1.0,
    green: 1.0,
    blue: 1.0,
    alpha: 1.0
};

#[allow(dead_code)]
impl TinColor {
    // TODO: provide a set implementation for hue, saturation, value
    //       these write methods should then mutate red, green, blue
    
    // hue - a value from 0-360 https://en.wikipedia.org/wiki/HSL_and_HSV
    //
    pub fn getHue(&self) -> Double {
        let mut h: Double = 0.0;

        let red = self.red;
        let green = self.green;
        let blue = self.blue;
        
        let maxM = red.max(blue.max(green));
        let minM = red.min(blue.min(green));
        let delta = maxM - minM;
        if delta < 0.00001 {
            return h
        }
        if maxM == red {
            h = (green - blue) / delta;
        }
        else if maxM == green {
            h = (blue - red) / delta + 2.0;
        }
        else {
            h = (red - green) / delta + 4.0;
        }
        h = h * 60.0;
        if h < 0.0 {
            h = h + 360.0;
        }
        return h
    }
    
    pub fn getSaturation(&self) -> Double {
        let mut s = 0.0;

        let red = self.red;
        let green = self.green;
        let blue = self.blue;

        let maxM = red.max(blue.max(green));
        let v = maxM;
        if maxM < 0.00001 {
            return s
        }
        let minM = red.min(blue.min(green));
        let delta = maxM - minM;
        s = delta / v;
        return s
    }
    
    pub fn getValue(&self) -> Double {
        let red = self.red;
        let green = self.green;
        let blue = self.blue;
        return red.max(blue.max(green));
    }
    
    pub fn new_from_rgba(red: Double, green: Double, blue: Double, alpha: Double) -> Self {
        Self {
            red: red,
            green: green,
            blue: blue,
            alpha: alpha
        }
    }
    
    
    pub fn new_from_pixel(pixel: TPixel) -> Self {
        let red = (pixel.red as Double) / 255.0;
        let green = (pixel.green as Double) / 255.0;
        let blue = (pixel.blue as Double) / 255.0;
        let alpha = (pixel.alpha as Double) / 255.0;
        TinColor::new_from_rgba(red, green, blue, alpha)
    }
    
    
    // TODO: add an init for HSV inital values, maybe for grayscale too?
    
    
    pub fn setFillColor(&self) {
        let r = &self.red;
        let g = &self.green;
        let b = &self.blue;
        let a = &self.alpha;
        fillColor_from_RGBA(r, g, b, a);
    }
    
    
    pub fn setStrokeColor(&self) {
        let r = &self.red;
        let g = &self.green;
        let b = &self.blue;
        let a = &self.alpha;
        strokeColor_from_RGBA(r, g, b, a);
    }
    
    
    
    // relative luminance https://en.wikipedia.org/wiki/Relative_luminance
    //
    pub fn luminance(&self) -> Double {
        return 0.2126 * self.red + 0.7152 * self.green + 0.0722 * self.blue;
    }
    
    
    pub fn lightness(&self) -> Double {
        let maxM = self.red.max(self.blue.max(self.green));
        let minM = self.red.min(self.blue.min(self.green));
        let l = 0.5 * (maxM + minM);
        return l
    }
    
    
    pub fn brightness(&self) -> Double {
        return (self.red + self.green + self.blue) / 3.0
    }
    
    
    
}