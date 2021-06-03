//
//  TScene.swift
//  Tin
//
//  Created by Loren Olson on 5/30/17.
//  Copyright © 2017 ASU. All rights reserved.
//

// The user should implement this for the TinScene
pub trait TScene {

    //fn new(&mut self);

    // setup() is called one time, immediately before first call to update().
    fn setup(&mut self);

    // Drawing code show go in update, or methods called during update.
    fn update(&mut self);
}

