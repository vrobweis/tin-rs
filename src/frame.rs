
type UInt = u32;

#[derive(Clone, Copy)]
pub struct TinFrame {
    width: u32,
    height: u32
}

impl Default for TinFrame {
    fn default() -> Self {
        Self {width: 600, height: 480}
    }
}
const MIN_WIDTH: UInt = 1;
const MIN_HEIGHT: UInt = 1;

impl TinFrame {

    

    pub fn new(width: u32, height: u32) -> Self {
        let mut new_frame = TinFrame::default();
        new_frame.set_width(width);
        new_frame.set_height(height);
        return new_frame;
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }
    pub fn set_width(&mut self, width: u32) {
        if width <= MIN_WIDTH {self.width = MIN_WIDTH} else {self.width = width}
    }
    
    pub fn get_height(&self) -> u32 {
        self.height
    }
    pub fn set_height(&mut self, height: u32) {
        if height <= MIN_HEIGHT {self.height = MIN_HEIGHT} else {self.height = height}
    }
}