//
//  TView.swift
//  Tin
//
//  Created by Loren Olson on 12/28/16.
//  Created at the School of Arts, Media and Engineering,
//  Herberger Institute for Design and the Arts,
//  Arizona State University.
//  Copyright (c) 2017 Arizona Board of Regents on behalf of Arizona State University
//
use super::{frame::TinFrame};

pub struct TinView {
    title: &'static str,
    target_framerate: u16,
    pub is_running: bool,

    frame: TinFrame,
    //infoFont: TinFont
}

impl TinView  {
    // MARK: - initializers
    
    /// TODO: Document this method.
    pub fn new(title: &'static str, frame: TinFrame) -> TinView {
        Self {
            title: title,
            target_framerate: 60 as u16,
            is_running: true,
            frame: frame,
            // FONT HERE
        }
        
    }
    
    /// TODO: Document this method.
    pub fn new_from_dimensions(title: &'static str, width: u32, height: u32) -> Self {
        //let newFrame = NSRect(x: 0.0, y: 0.0, width: width, height: height);
        let frame = TinFrame::new(width, height);
        Self::new(title, frame)
    }

    /// TODO: Document this method.
    pub fn get_fps(&self) -> u16 {
        self.target_framerate
    }

    /// TODO: Document this method.
    pub fn set_fps(&mut self, fps: u16) {
        self.target_framerate = fps;
    }

    pub fn get_frame(&self) -> &TinFrame {
        &self.frame
    }

    pub fn get_title(&self) -> &str {
        self.title
    }


    /// move the window to the top, left corner of the current screen
    pub fn move_window_to_topleft(&mut self) {
        todo!("Moving window to top left of current screen through this method is not yet supported.");
    }
    
    
    /// move the window to center it in the current screen
    pub fn move_window_to_center(&mut self) {
        todo!("Moving window to center of screen through this method is not yet supported.");
    }
}

    