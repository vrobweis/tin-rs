
extern crate image;
use image::{DynamicImage, GenericImageView, ImageError};

use crate::{Double, UInt, color::TColor, draw::{image, image_with_size}};

pub struct TinImage {
    image: DynamicImage,
    pub(crate) width: UInt,
    pub(crate) height: UInt
}

impl TinImage {
    
    pub fn new_from_image(image: DynamicImage) -> Self {
        Self {
            width: image.width(),
            height: image.height(),
            image,
        }
    }
    
    pub fn new_from_file_path(file_path: String) -> Result<Self, ImageError> {
        let image = image::open(file_path)?;
        Ok(Self::new_from_image(image))
    }
    
    
    pub fn color(&self, at_x: UInt, at_y: UInt) -> impl TColor {
        let p = self.image.get_pixel(at_x, at_y);
        return UInt::new_from_rgba(p.0[0] as Double, p.0[1] as Double, p.0[2] as Double, p.0[3] as Double);
    }
    
    pub fn get_width(&self) -> UInt {
        self.width
    }

    pub fn get_height(&self) -> UInt {
        self.height
    }

    pub fn draw(&self, x: Double, y: Double) {
        image(self, x, y);
    }
    
    pub fn draw_with_size(&self, x: Double, y: Double, width: Double, height: Double) {
        image_with_size(self, x, y, width, height);
    }
}