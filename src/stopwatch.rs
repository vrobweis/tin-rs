use std::time::{Duration, Instant};

//
//  TStopwatch.swift
//  Tin
//
//  Created by Loren Olson on 1/5/17.
//  Created at the School of Arts, Media and Engineering,
//  Herberger Institute for Design and the Arts,
//  Arizona State University.
//  Copyright (c) 2017 Arizona Board of Regents on behalf of Arizona State University
//

/**
  A struct for simple timing.
  Uses mach_absolute_time for reliable, fine-grained timing.
 */
pub struct TStopwatch {
    start_time: Instant
}



impl TStopwatch {
    pub fn get_elapsed_time(&self) -> Duration {
        self.start_time.elapsed()
    }
    
    pub fn get_elapsed_nano_seconds(&self) -> u128 {
        self.get_elapsed_time().as_nanos()
    }
    
    pub fn get_elapsed_seconds(&self) -> f64 {
        self.get_elapsed_time().as_secs_f64()
    }
    
    
    // The start time is set on initialization
    pub fn new() -> Self {
        Self {
            start_time: Instant::now()
        }
    }
    
    
    // Reset the start time
    pub fn reset(&mut self) {
        self.start_time = Instant::now();
    }
    
    
}