use std::time::{Duration, Instant};

use crate::{Double, Float, UInt, ULong, UShort};

/**
 A struct for simple timing.
 Uses mach_absolute_time for reliable, fine-grained timing.
*/
pub struct TStopwatch {
    start_time: Instant,
}

impl TStopwatch {
    pub fn get_elapsed_time(&self) -> Duration {
        self.start_time.elapsed()
    }

    pub fn get_elapsed_nano_secs(&self) -> u128 {
        self.get_elapsed_time().as_nanos()
    }

    pub fn get_elapsed_secs(&self) -> Double {
        self.get_elapsed_time().as_secs_f64()
    }

    pub fn get_elapsed_millisecs(&self) -> u128 {
        self.get_elapsed_time().as_millis()
    }

    // The start time is set on initialization
    pub fn start() -> Self {
        Self {
            start_time: Instant::now(),
        }
    }

    // Reset the start time
    pub fn reset(&mut self) {
        self.start_time = Instant::now();
    }

    pub fn stop(self) {}
}

use std::thread::sleep;
pub(crate) fn pace_frames(target_fps: UShort, last_frame_time: Instant) {
    let stopwatch = TStopwatch {
        start_time: last_frame_time,
    };

    let frametime_ms: UInt = match target_fps as UInt {
        // Prevent divide by zero
        0 => 1000 / 1,
        _ => 1000 / target_fps as UInt,
    };

    let safe_sleep_period_ms = frametime_ms as Float * (9_f32 / 10_f32);
    let safe_sleep_period: Duration = Duration::from_millis(safe_sleep_period_ms as ULong);

    // TODO: Use stopwatch.rs methods to replicate this functionality

    if stopwatch.get_elapsed_millisecs() < safe_sleep_period.as_millis() {
        sleep(safe_sleep_period);
    }

    while (stopwatch.get_elapsed_millisecs() as UInt) < frametime_ms {
        // Spinlock for whatever time is left after the thread sleep function
    }
}
