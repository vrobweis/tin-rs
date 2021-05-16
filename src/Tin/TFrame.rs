#[derive(Clone, Copy)]
pub struct TinFrame {
    width: u32,
    height: u32
}

#[allow(dead_code)]
impl TinFrame {
    pub fn new(width: u32, height: u32) -> Self {
        let w: u32;
        let h: u32;
        if width <= 0 {w = 0} else {w = width};
        if height <= 0 {h = 0} else {h = height};
        Self {
            width: w,
            height: h
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }
    pub fn set_width(&mut self, width: u32) {
        if width <= 0 {self.width = 0} else {self.width = width}
    }
    
    pub fn get_height(&self) -> u32 {
        self.height
    }
    pub fn set_height(&mut self, height: u32) {
        if height <= 0 {self.height = 0} else {self.height = height}
    }
}