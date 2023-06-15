use crate::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CustomMove {
    pub points: u8,
    pub destination: Point,
}

impl CustomMove {
    pub fn new(points: u8, destination: Point) -> Self {
        return Self {
            points,
            destination,
        };
    }
}
