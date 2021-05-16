extern crate rand;

use std::{sync::Mutex, thread::Thread};

use rand::{Rng, distributions::{self, Standard}, prelude::{Distribution, ThreadRng}};

//
//  TRandom.swift
//  Tin
//
//  Created by Loren Olson on 11/3/16.
//  Created at the School of Arts, Media and Engineering,
//  Herberger Institute for Design and the Arts,
//  Arizona State University.
//  Copyright (c) 2017 Arizona Board of Regents on behalf of Arizona State University
//

use super::{Double, Float};


// TODO: Implement static vars here that can be globally used to produce random values

pub struct TRandom {
    randomSource: ThreadRng // GKLinearCongruentialRandomSource.sharedRandom() NEED TO GENERATE A RANDOM VALUE TO USE   
}

// pub static TinRandom: TRandom = TRandom {randomSource: ThreadRng::default()};
#[allow(dead_code)]
impl TRandom {
    /// return a random Double in the range [0.0, max].
    pub fn next_from_zeroToMax(max: Double) -> Double {
        todo!("randomGaussian not implemented in TRandom");
        // return Double(self.randomSource.nextUniform()) * max;
    }
    
    /// return a random Double in the range [min, max].
    pub fn next_from_minToMax(min: Double, max: Double) -> Double {
        todo!("randomGaussian not implemented in TRandom");
        let distance = max - min;
        // return min + Double(self.randomSource.nextUniform()) * distance
    }
}


// static mut tin_random_source_mutex: Mutex<ThreadRng> = Mutex::new(ThreadRng::default());

// static tinGaussianSource:Double = GKGaussianDistribution(lowestValue: 0, highestValue: 10000)

#[allow(dead_code)]
pub fn random_from_zero(max: Double) -> Double {
    return ThreadRng::default().gen::<Double>() * max;
}

#[allow(dead_code)]
pub fn random(min: Double, max: Double) -> Double {
    let distance = max - min;
    return min + ThreadRng::default().sample(rand::distributions::Uniform::new(min, max)) * distance
}

#[allow(dead_code)]
/** 
Return a random value in a normal distribution, in the range -1 to 1.
The mean value is 0. Deviation is 0.3334. 
*/
pub fn randomGaussian() -> Double {
    todo!("randomGaussian not implemented in TRandom");
    // return (Double(tinGaussianSource.nextUniform()) * 2.0) - 1.0
}