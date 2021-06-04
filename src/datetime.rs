
use chrono::{Datelike, Timelike, prelude::Utc};
use crate::{Int, Long, UInt};


pub fn year() -> Int {
    Utc::now().year()
}

pub fn month() -> UInt {
    Utc::now().month()
}

pub fn day() -> UInt {
    Utc::now().day()
}

pub fn hour() -> UInt {
    Utc::now().hour()
}

pub fn minute() -> UInt {
    Utc::now().minute()
}

pub fn second() -> UInt {
    Utc::now().second()
}

pub fn millis() -> Long {
    Utc::now().timestamp_millis()
}

pub fn nanosecond() -> UInt {
    Utc::now().nanosecond()
}

