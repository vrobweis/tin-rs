#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unreachable_code)]
#![allow(unused_variables)]
mod TinApp;

mod Tin;

use Tin::{TController::TinController, TFrame::TinFrame, TScene::{TScene}, TShapes::TinShape, TVertex::TinVertex, TView::{TView, TinView}};

extern crate lazy_static;

use std::collections::VecDeque as Queue;

use TinApp::{ App};

use crate::Tin::TColor::TinColor;


struct Scene {
    timeElapsed: f64
}

impl TScene for Scene {

    fn setup(&mut self) {
        println!("Scene::setup");
        Tin::background(0.5, 0.5, 0.5);
    }

    fn update(&mut self) {
        println!("Scene::update");
        const rateOfChange: f64 = 0.001;

        let color: TinColor = TinColor::new_from_rgba(1.0, 0.1, 0.1, 1.0);
        Tin::fillColor_from_color(&color);
        Tin::triangle(-0.3 + self.timeElapsed, -0.3, 0.0, 1.0 + self.timeElapsed, 0.3, -0.3);
        self.timeElapsed = self.timeElapsed - rateOfChange;
    }
}

fn main() {

    let frame = TinFrame::new(600, 500);

    let scene = Box::new(Scene{timeElapsed: 0.0});
    let mut AViewToUse = Box::new(TinView::new(frame));
    AViewToUse.present(scene);

    let controller = Box::new(TinController::new(AViewToUse));

    

    let mut app = TinApp::App::new_with_controller("title", controller);

    std::process::exit(
        match app.run() {
            Ok(_) => 0,
            Err(err) => {
                eprintln!("error: {:?}", err);
                1
            }
        }
    );
}


