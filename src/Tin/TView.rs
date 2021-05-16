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

use panic;

use luminance_windowing::{self, WindowDim, WindowOpt, CursorMode};
use super::{TScene::{TScene}, TFrame::TinFrame, get_tin};



use super::{Double, Float};

trait TViewDelegate {
    fn update();
}


pub struct TinView {
    //timer: Timer,
    Target_Framerate: u16,
    pub Is_Running: bool,



    frame: TinFrame,
    //infoFont: TinFont,
    pub scene: Option<Box<dyn TScene>>
}

impl TinView  {
    // MARK: - initializers
    
    /// TODO: Document this method.
    pub fn new<'n>(frame: TinFrame) -> TinView {
        Self {
            Target_Framerate: 60 as u16,
            Is_Running: true,
            frame: frame,
            // FONT HERE
            scene: Option::None,//SOMETHING HERE
        }
        
    }
    
    /// TODO: Document this method.
    pub fn new_from_dimensions(width: u32, height: u32) -> Self {
        //let newFrame = NSRect(x: 0.0, y: 0.0, width: width, height: height);
        let frame = TinFrame::new(width, height);
        Self::new(frame)
    }

}

pub trait TView  {

    /// TODO: Document this method.
    fn present(&mut self, scene: Box<dyn TScene>);

    /// TODO: Document this method.
    fn draw(&mut self);

    /// TODO: Document this method.
    fn stopUpdates(&mut self) ;
    
    /// TODO: Document this method.
    fn startUpdates(&mut self) ;

    /// TODO: Document this method.
    fn updateView(&mut self) {
        // needsDisplay = true
    }

    fn get_fps(&self) -> u16;
    fn set_fps(&mut self, fps: u16); 
    
    fn get_dimensions(&self) -> TinFrame;
}

impl TView for TinView {

    /// TODO: Document this method.
    fn present(&mut self, mut scene: Box<dyn TScene>) {
        scene.setup();
        self.scene = Option::Some(scene);
    }

    /// TODO: Document this method.
    fn draw(&mut self) {
        
        {
            let mut tin = get_tin();
            tin.updateFrameCount();
            tin.prepareForUpdate(self.frame);
        }
        
        /*if self.scene.needsSetup {
            self.scene.setup();
            self.scene.needsSetup = false;
        }*/
        

        match &mut self.scene {
            Some(s) => {s.update()}
            None => {panic!("No scene :(")}
        }
        
        // Performance debugging display should be rendered here

        super::get_tin().didFinishUpdate();
        
    }

    /// TODO: Document this method.
    fn stopUpdates(&mut self) {
        todo!("Stop timer for TView not implemented.");
    }
    
    /// TODO: Document this method.
    fn startUpdates(&mut self) {
        todo!("Start timer for TView not implemented.");
    }

    /// TODO: Document this method.
    fn get_fps(&self) -> u16 {
        self.Target_Framerate
    }

    /// TODO: Document this method.
    fn set_fps(&mut self, fps: u16) {
        self.Target_Framerate = fps;
    }

    fn get_dimensions(&self) -> TinFrame {
        self.frame
    }
}


/*
open class TView: NSView {
    
    
    
    
    
    
    private var timer: Timer?
    
    public func stopUpdates() {
        if timer != nil {
            timer!.invalidate()
            timer = nil
        }
    }
    
    
    public func startUpdates() {
        if timer == nil {
            startUpdateTimer()
        }
    }
    
    
    private func startUpdateTimer() {
        debug("Start updates @ \(frameRate) fps")
        let interval = 1.0 / frameRate
        timer = Timer.scheduledTimer(timeInterval: interval, target: self, selector: #selector(TView.updateView), userInfo: nil, repeats: true)
    }
*/
    
    // MARK: - NSView
    
    


    
    
    
    
    
    
    