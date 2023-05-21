use crate::prelude::*;

#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}


impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        return Point {
            x,
            y
        };
    }
}