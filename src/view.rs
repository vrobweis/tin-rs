use crate::{frame::TinFrame, UInt};

pub struct TinView {
    title: &'static str,
    frame: TinFrame,
}

pub trait TView {
    /// TODO: Document this method.
    fn new(title: &'static str, frame: TinFrame) -> Self;

    /// TODO: Document this method.
    fn from_dimensions(title: &'static str, width: UInt, height: UInt) -> Self;

    /// TODO: Document this method.
    fn get_frame(&self) -> &TinFrame;
    /// TODO: Document this method.
    fn get_title(&self) -> &str;
}

impl TView for TinView {
    fn new(title: &'static str, frame: TinFrame) -> TinView {
        Self { title, frame }
    }

    fn from_dimensions(title: &'static str, width: UInt, height: UInt) -> Self {
        //let newFrame = NSRect(x: 0.0, y: 0.0, width: width, height: height);
        let frame = TinFrame::new(width, height);
        Self::new(title, frame)
    }

    fn get_frame(&self) -> &TinFrame {
        &self.frame
    }

    fn get_title(&self) -> &str {
        self.title
    }
}

impl TinView {
    // MARK: - initializers

    /// move the window to the top, left corner of the current screen
    pub fn move_window_to_topleft(&mut self) {
        todo!(
            "Moving window to top left of current screen through this method is not yet supported."
        );
    }

    /// move the window to center it in the current screen
    pub fn move_window_to_center(&mut self) {
        todo!("Moving window to center of screen through this method is not yet supported.");
    }
}
