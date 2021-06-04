//
//  TinImage<'a>.swift
//  Tin
//
//  Created by Loren Olson on 1/3/17.
//  Created at the School of Arts, Media and Engineering,
//  Herberger Institute for Design and the Arts,
//  Arizona State University.
//  Copyright (c) 2017 Arizona Board of Regents on behalf of Arizona State University
//
extern crate image;
use image::{DynamicImage, GenericImageView, ImageError};

use crate::{Double, UInt, color::TColor, draw::{image, image_with_size}};

pub struct TinImage {
    image: DynamicImage,
    pub width: UInt,
    pub height: UInt
}

impl TinImage {
    
    pub fn new_from_image(image: DynamicImage) -> Self {
        let w = image.width();
        let h = image.height();
        Self {
            image: image,
            width: w,
            height: h
        }
    }
    
    pub fn new_from_file(file_path: String) -> Result<Self, ImageError> {
        let img = image::open(file_path);
        let image;
        
        match img  {
            Ok(i) => image = i,
            Err(e) => return Err(e)
        }
        Ok(
            {
                let w = image.width();
                let h = image.height();
                Self {
                    image: image,
                    width: w,
                    height: h
                }
            }
        )
    }
    
    
    pub fn color(&self, at_x: UInt, at_y: UInt) -> impl TColor {
        let p = self.image.get_pixel(at_x, at_y);
        return UInt::new_from_rgba(p.0[0] as Double, p.0[1] as Double, p.0[2] as Double, p.0[3] as Double);
    }
    
    
    
    //func image(image: TinImage<'a>, x: Double, y: Double)
    //func image(image: TinImage<'a>, x: Double, y: Double, width: Double, height: Double)

    pub fn draw(&self, x: &Double, y: &Double) {
        image(self, x, y);
    }
    
    pub fn draw_with_size(&self, x: &Double, y: &Double, width: &Double, height: &Double) {
        image_with_size(self, x, y, width, height);
    }
}