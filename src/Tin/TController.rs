use super::{TPoint::TinPoint, get_tin, TView::{TinView, TView}, TScene::TScene};

//
//  TController.swift
//  Tin
//
//  Created by Loren Olson on 5/15/17.
//  Copyright © 2017 ASU. All rights reserved.
//
use super::{Double, Float};

pub struct TinController {
    tin_view: Box<dyn TView>,
}

impl Default for TinController {
    fn default() -> Self {
        Self {
            tin_view: Box::new(TinView::new_from_dimensions(500, 500))
        }
    }
}

impl TinController {
    pub fn new(view: Box<dyn TView>) -> Self {
        Self {
            tin_view: view
        }
    }
}

impl TController for TinController {
    fn get_view(&mut self) -> &mut Box<dyn TView> {
        &mut self.tin_view
    }

    fn viewWillAppear(&mut self) {
        todo!()
    }

    fn viewDidAppear(&mut self) {
        todo!()
    }

    fn updateViewConstraints(&mut self) {
        todo!()
    }

    fn viewWillLayout(&mut self) {
        todo!()
    }

    fn viewDidLayout(&mut self) {
        todo!()
    }

    fn viewWillDisappear(&mut self) {
        todo!()
    }

    fn viewDidDisappear(&mut self) {
        todo!()
    }
}

pub trait TController {

    fn get_view(&mut self) -> &mut Box<dyn TView>;

    fn viewWillAppear(&mut self);
    fn viewDidAppear(&mut self);
    fn updateViewConstraints(&mut self);
    fn viewWillLayout(&mut self);
    fn viewDidLayout(&mut self);
    fn viewWillDisappear(&mut self);
    fn viewDidDisappear(&mut self);

    fn controllerLOOP(&mut self) {
        self.viewWillAppear();
        self.viewDidAppear();
        self.updateViewConstraints();
        self.viewWillLayout();
        self.viewDidLayout();
        self.viewWillDisappear();
        self.viewDidDisappear();
    }





    /// move the window to the top, left corner of the current screen
    fn moveWindowToTopLeft(&mut self) {
        todo!("Moving window to top left of current screen through this method is not yet supported.");
    }
    
    
    /// move the window to center it in the current screen
    fn moveWindowToCenter(&mut self) {
        todo!("Moving window to center of screen through this method is not yet supported.");
    }

    
    
    
    
    /// create a TView object and assign to the view property
    fn makeView(&mut self, width: Double, height: Double) {
        todo!("makeView not supported.");
        // view = TView(width: width, height: height);
    }
    
    
    /// make scene be the current TScene that is displayed by this TController.
    fn present(& mut self, scene: Box<dyn TScene>) {
        todo!("Moving window to top left through this method is not yet supported.");
        let tv= self.get_view();        
        //self.get_view().scene
        tv.present(scene);
    }


// _______________________ NEEDS WORK
    
    
    fn keyDown(&mut self, key: char) {
        todo!("Tin controller input handler not supported");
    }


    fn keyUp(&mut self, key: char) {
        todo!("Tin controller input handler not supported");
    }


    // If the user overrides any of these mouse responder events in a TController subclass,
    // then it is important that they call the super method, otherwise mouse position will break.

    fn mouseDown(&mut self, pos: [u32;2]) {
        todo!("Tin controller input handler not supported");
        let point: TinPoint;
        let tin = get_tin();
        tin.mouseMoved(point);
        tin.mousePressed = true;
    }



    fn mouseDragged(&mut self, pos: [u32;2]) {
        todo!("Tin controller input handler not supported");
        let point: TinPoint;
        let tin = get_tin();
        tin.mouseMoved(point);
    }



    fn mouseMoved(&mut self, pos: [u32;2]) {
        todo!("Tin controller input handler not supported");
        let point: TinPoint;
        let tin = get_tin();
        tin.mouseMoved(point);
    }


    fn mouseUp(&mut self, pos: [u32;2]) {
        todo!("Tin controller input handler not supported");
        let point: TinPoint;
        let tin = get_tin();
        tin.mouseMoved(point);
        tin.mousePressed = false;
    }


    fn rightMouseDown(&mut self, pos: [u32;2]) {
        todo!("Tin controller input handler not supported");
        let point: TinPoint;
        let tin = get_tin();
        tin.mouseMoved(point);
        tin.mousePressed = true;
    }


    fn rightMouseDragged(&mut self, pos: [u32;2]) {
        todo!("Tin controller input handler not supported");
        let point: TinPoint;
        let tin = get_tin();
        tin.mouseMoved(point);
    }


    fn rightMouseUp(&mut self, pos: [u32;2]) {
        todo!("Tin controller input handler not supported");
        let point: TinPoint;
        let tin = get_tin();
        tin.mouseMoved(point);
        tin.mousePressed = false;
    }


    fn otherMouseDown(&mut self, pos: [u32;2]) {
        todo!("Tin controller input handler not supported");
        let point: TinPoint;
        let tin = get_tin();
        tin.mouseMoved(point);
        tin.mousePressed = true;
    }


    fn otherMouseDragged(&mut self, pos: [u32;2]) {
        todo!("Tin controller input handler not supported");
        let point: TinPoint;
        let tin = get_tin();
        tin.mouseMoved(point);
    }


    fn otherMouseUp(&mut self, pos: [u32;2]) {
        todo!("Tin controller input handler not supported");
        let point: TinPoint;
        let tin = get_tin();
        tin.mouseMoved(point);
        tin.mousePressed = false;
    }








}


// open class TController: NSViewController {
//    override open var view: NSView {
//        get {
//            print("get view")
//            return tinView
//        }
//        set {
//            print("set view")
//            if newValue is TView {
//                print("set tinView using newValue")
//                tinView = newValue as! TView
//            }
//            else {
//                print("Warning, incorrect view type")
//            }
//        }
//    }
