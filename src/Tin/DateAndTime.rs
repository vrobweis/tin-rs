//
//  DateAndTime.rs
//  Tin
//  
//  Adapted by Vincent Weis on 5/9/2021
//_____________________________________
//  Original Swift version:
//  DateAndTime.swift
//  Tin
//
//  Created by Loren Olson on 1/3/17.
//  Created at the School of Arts, Media and Engineering,
//  Herberger Institute for Design and the Arts,
//  Arizona State University.
//  Copyright (c) 2017 Arizona Board of Regents on behalf of Arizona State University
//
//  Global functions for simple date and time calculations.

#![allow(dead_code)]

use chrono::{Datelike, Timelike, prelude::Utc};

type Int = i32;
type UInt = u32;
type LongInt = i64;

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


pub fn nanosecond() -> UInt {
    Utc::now().nanosecond()
}


pub fn millis() -> LongInt {
    Utc::now().timestamp_millis()
}