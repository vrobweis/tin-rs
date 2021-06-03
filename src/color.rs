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

use super::{calculation::{constrain}, draw::{fill_color_from_rgba, stroke_color_from_rgba}, Double};

pub struct TPixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8
}

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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TinColor {
    pub red: Double,
    pub green: Double,
    pub blue: Double,
    pub alpha: Double
}

impl TinColor {
    pub fn get_u32_repr(&self) -> u32 {
        u32::new_from_rgba(self.get_red(), self.get_green(), self.get_blue(), self.get_alpha())
    }
}


pub(crate) const DEFAULT_COLOR_FILL: TinColor = TinColor {
    red: 0.7,
    green: 0.7,
    blue: 0.7,
    alpha: 1.0
};
pub(crate) const DEFAULT_COLOR_STROKE: TinColor = TinColor {
    red: 0.1,
    green: 0.1,
    blue: 0.1,
    alpha: 1.0
};
pub(crate) const DEFAULT_COLOR_BACKGROUND: TinColor = TinColor {
    red: 1.0,
    green: 1.0,
    blue: 1.0,
    alpha: 1.0
};


impl TColor for TinColor {
    
    fn new_from_rgba(red: Double, green: Double, blue: Double, alpha: Double) -> Self {
        Self {
            red: red,
            green: green,
            blue: blue,
            alpha: alpha
        }
    }

    fn get_red(&self) -> Double {
        self.red
    }

    fn get_green(&self) -> Double {
        self.green
    }

    fn get_blue(&self) -> Double {
        self.blue
    }

    fn get_alpha(&self) -> Double {
        self.alpha
    }

}



use crate::calculation::*;
impl TColor for u32 {
    fn new_from_rgba(red: Double, green: Double, blue: Double, alpha: Double) -> Self {
        let mut value: Self = 0x0;
        let mut values: [u8; 4] = [0,0,0,0];
        let rgba = [red,green,blue,alpha];
        for i in 0..rgba.len() {
            let mut v = rgba[i];
            v = constrain(v, 0.0, 1.0);
            v = remap(v, 0.0, 1.0, 0.0, 255.0);
            values[i] = v as u8;
        }
        for i in 0..values.len() {
            let index_to_assign = values.len() - 1 - i;
            let offset = 8 * i;
            value = value + ((values[index_to_assign] as Self) << offset);
        }
        return value;
    }

    fn get_red(&self) -> Double {
        let r_u8 = (*self & 0xFF000000_u32) >> (8*3) as u8;
        remap(r_u8 as Double, 0.0, 255.0, 0.0, 1.0)
    }

    fn get_green(&self) -> Double {
        let r_u8 = (*self & 0xFF0000_u32) >> (8*2) as u8;
        remap(r_u8 as Double, 0.0, 255.0, 0.0, 1.0)
    }

    fn get_blue(&self) -> Double {
        let r_u8 = (*self & 0xFF00_u32) >> (8*1) as u8;
        remap(r_u8 as Double, 0.0, 255.0, 0.0, 1.0)
    }

    fn get_alpha(&self) -> Double {
        let r_u8 = (*self & 0xFF_u32) >> (8*0) as u8;
        remap(r_u8 as Double, 0.0, 255.0, 0.0, 1.0)
    }
}

pub trait TColor {

    fn new_from_rgba(red: Double, green: Double, blue: Double, alpha: Double) -> Self;
    fn new_from_pixel(pixel: TPixel) -> Self where Self: Sized {
        let red = (pixel.red as Double) / 255.0;
        let green = (pixel.green as Double) / 255.0;
        let blue = (pixel.blue as Double) / 255.0;
        let alpha = (pixel.alpha as Double) / 255.0;
        Self::new_from_rgba(red, green, blue, alpha)
    }

    fn get_red(&self) -> Double;
    fn get_green(&self) -> Double;
    fn get_blue(&self) -> Double;
    fn get_alpha(&self) -> Double;



    // TODO: provide a set implementation for hue, saturation, value
    //       these write methods should then mutate red, green, blue
    
    // hue - a value from 0-360 https://en.wikipedia.org/wiki/HSL_and_HSV
    //
    fn get_hue(&self) -> Double {
        let mut h: Double = 0.0;

        let red = self.get_red();
        let green = self.get_green();
        let blue = self.get_blue();
        
        let max_m = red.max(blue.max(green));
        let min_m = red.min(blue.min(green));
        let delta = max_m - min_m;
        if delta < 0.00001 {
            return h
        }
        if max_m == red {
            h = (green - blue) / delta;
        }
        else if max_m == green {
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
    
    fn get_saturation(&self) -> Double {
        let mut s = 0.0;

        let red = self.get_red();
        let green = self.get_green();
        let blue = self.get_blue();

        let max_m = red.max(blue.max(green));
        let v = max_m;
        if max_m < 0.00001 {
            return s
        }
        let min_m = red.min(blue.min(green));
        let delta = max_m - min_m;
        s = delta / v;
        return s
    }
    
    fn get_value(&self) -> Double {
        let red = self.get_red();
        let green = self.get_green();
        let blue = self.get_blue();
        return red.max(blue.max(green));
    }

    // TODO: add an init for HSV inital values, maybe for grayscale too?
    
    
    fn set_fill_color(&self) {
        let r = &self.get_red();
        let g = &self.get_green();
        let b = &self.get_blue();
        let a = &self.get_alpha();
        fill_color_from_rgba(r, g, b, a);
    }
    
    
    fn set_stroke_color(&self) {
        let r = &self.get_red();
        let g = &self.get_green();
        let b = &self.get_blue();
        let a = &self.get_alpha();
        stroke_color_from_rgba(r, g, b, a);
    }
    
    
    
    // relative luminance https://en.wikipedia.org/wiki/Relative_luminance
    //
    fn luminance(&self) -> Double {
        return 0.2126 * self.get_red() + 0.7152 * self.get_green() + 0.0722 * self.get_blue();
    }
    
    
    fn lightness(&self) -> Double {
        let max_m = self.get_red().max(self.get_blue().max(self.get_green()));
        let min_m = self.get_red().min(self.get_blue().min(self.get_green()));
        let l = 0.5 * (max_m + min_m);
        return l
    }
    
    
    fn brightness(&self) -> Double {
        return (self.get_red() + self.get_green() + self.get_blue()) / 3.0
    }
}