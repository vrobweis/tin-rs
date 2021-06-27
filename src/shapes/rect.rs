use crate::{
    brush::TBrush,
    context::DrawState,
    point::{TPoint, TinPoint},
    Double,
};

#[derive(Debug, Clone)]
pub struct TinRect {
    pub center: TinPoint,
    width: Double,
    height: Double,
}

impl TinRect {
    pub fn from_dimensions(x: Double, y: Double, w: Double, h: Double) -> Self {
        Self {
            center: TinPoint::from_coords(x, y),
            width: w.abs(),
            height: h.abs(),
        }
    }

    pub fn from_dimensions_and_point(center: TinPoint, w: Double, h: Double) -> Self {
        Self {
            center,
            width: w.abs(),
            height: h.abs(),
        }
    }

    pub fn get_width(&self) -> Double {
        self.width
    }
    pub fn set_width(&mut self, width: Double) {
        self.width = width.abs()
    }

    pub fn get_height(&self) -> Double {
        self.height
    }
    pub fn set_height(&mut self, height: Double) {
        self.height = height.abs()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct TinRoundedRect {
    pub rect: TinRect,
    pub radius_x: Double,
    pub radius_y: Double,
}

impl TinRoundedRect {
    pub fn new(rect: TinRect, radius_x: Double, radius_y: Double) -> Self {
        Self {
            rect,
            radius_x,
            radius_y,
        }
    }
}

pub(crate) trait RectRenderer {
    fn rect(&mut self, center: TinPoint, w: Double, h: Double, brush: TBrush, state: DrawState) {
        self.rect_with_tinrect(
            &TinRect::from_dimensions_and_point(center, w, h),
            brush,
            state,
        );
    }
    fn rect_with_tinrect(&mut self, with_rect: &TinRect, brush: TBrush, state: DrawState);
    fn rounded_rect(&mut self, rounded_rect: &TinRoundedRect, brush: TBrush, state: DrawState);
}
