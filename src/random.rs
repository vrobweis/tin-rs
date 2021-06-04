extern crate rand;

use rand::{Rng, prelude::{ThreadRng}};
use crate::{Double};


// TODO: Implement static vars here that can be globally used to produce random values

pub struct TRandom {
    random_source: ThreadRng // GKLinearCongruentialRandomSource.sharedRandom() NEED TO GENERATE A RANDOM VALUE TO USE   
}

// pub static TinRandom: TRandom = TRandom {random_source: ThreadRng::default()};

impl TRandom {
    /// return a random Double in the range [0.0, max].
    pub fn next_from_zero_to_max(&mut self, max: Double) -> Double {
        self.random_source.gen_range(0.0..max)
        // return Double(self.random_source.nextUniform()) * max;
    }
    
    #[allow(unreachable_code)]
    /// return a random Double in the range [min, max].
    pub fn next_from_min_to_max(&mut self, min: Double, max: Double) -> Double {
        let distance = max - min;
        return min + self.next_from_zero_to_max(distance)
    }
}


// static mut tin_random_source_mutex: Mutex<ThreadRng> = Mutex::new(ThreadRng::default());

// static tinGaussianSource:Double = GKGaussianDistribution(lowestValue: 0, highestValue: 10000)


pub fn random_from_zero(max: Double) -> Double {
    return ThreadRng::default().gen::<Double>() * max;
}


pub fn random(min: Double, max: Double) -> Double {
    let distance = max - min;
    return min + ThreadRng::default().sample(rand::distributions::Uniform::new(min, max)) * distance
}


/** 
Return a random value in a normal distribution, in the range -1 to 1.
The mean value is 0. Deviation is 0.3334. 
*/
pub fn random_gaussian() -> Double {
    todo!("randomGaussian not implemented in TRandom");
    // return (Double(tinGaussianSource.nextUniform()) * 2.0) - 1.0
}