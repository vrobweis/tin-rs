use image::{DynamicImage, GenericImageView, ImageError};

use crate::{
    color::TColor,
    draw::{image, image_with_size},
    point::TinPoint,
    Double, UInt,
};

#[derive(Debug)]
pub struct TinImage {
    pub(crate) image: DynamicImage,
}

impl TinImage {
    pub fn from_image(image: DynamicImage) -> Self {
        Self { image }
    }

    pub fn from_file_path(file_path: String) -> Result<Self, ImageError> {
        let image = image::open(file_path)?;
        Ok(Self::from_image(image))
    }

    pub fn color(&self, at_x: UInt, at_y: UInt) -> impl TColor {
        let p = self.image.get_pixel(at_x, at_y);
        return UInt::from_rgba(
            p.0[0] as Double,
            p.0[1] as Double,
            p.0[2] as Double,
            p.0[3] as Double,
        );
    }

    pub fn color_rows<T: TColor>(&self) -> Vec<Vec<T>> {
        let (width, height) = (self.image.width(), self.image.height());
        let rows = (0..height).into_iter()
            .map(
                |y| {
                    let mut row = Vec::with_capacity(width as usize);
                    for x in 0..width {
                        row.push(
                            {
                                let p = self.image.get_pixel(x, y);
                                T::from_rgba(
                                    p.0[0] as Double,
                                    p.0[1] as Double,
                                    p.0[2] as Double,
                                    p.0[3] as Double,
                                )
                            }
                        )
                    };
                    row
                }
            ).collect();
        return rows
    }

    pub fn get_width(&self) -> UInt {
        self.image.width()
    }

    pub fn get_height(&self) -> UInt {
        self.image.height()
    }

    pub fn draw(&'static self, x: Double, y: Double) {
        image(self, x, y);
    }

    pub fn draw_with_size(&'static self, x: Double, y: Double, width: Double, height: Double) {
        image_with_size(self, x, y, width, height);
    }
}

pub(crate) trait ImageRenderer {
    fn image(&mut self, image: &TinImage, center: TinPoint, state: crate::context::DrawState) {
        self.image_with_size(
            image,
            center,
            image.get_width() as Double,
            image.get_height() as Double,
            state,
        )
    }

    fn image_with_size(
        &mut self,
        image: &TinImage,
        center: TinPoint,
        width: Double,
        height: Double,
        state: crate::context::DrawState,
    ) {
        self.image_with_size_and_resize(image, center, width, height, false, state);
    }

    fn image_with_size_and_resize(
        &mut self,
        image: &TinImage,
        center: TinPoint,
        width: Double,
        height: Double,
        resize: bool,
        state: crate::context::DrawState,
    );
}
