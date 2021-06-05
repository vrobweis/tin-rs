
use crate::UInt;

#[derive(Debug, Clone, Copy)]
pub struct TinFrame {
    width: UInt,
    height: UInt
}

const DEFAULT_WIDTH: UInt = 600;
const DEFAULT_HEIGHT: UInt = 480;

impl Default for TinFrame {
    fn default() -> Self {
        Self {width: DEFAULT_WIDTH, height: DEFAULT_HEIGHT}
    }
}

const MIN_WIDTH: UInt = 1;
const MIN_HEIGHT: UInt = 1;

impl TinFrame {

    pub fn new(width: UInt, height: UInt) -> Self {
        let mut new_frame = TinFrame::default();
        new_frame.set_width(width);
        new_frame.set_height(height);
        return new_frame;
    }

    pub fn get_width(&self) -> UInt {
        self.width
    }
    pub fn set_width(&mut self, width: UInt) {
        if width <= MIN_WIDTH {self.width = MIN_WIDTH} else {self.width = width}
    }
    
    pub fn get_height(&self) -> UInt {
        self.height
    }
    pub fn set_height(&mut self, height: UInt) {
        if height <= MIN_HEIGHT {self.height = MIN_HEIGHT} else {self.height = height}
    }
}