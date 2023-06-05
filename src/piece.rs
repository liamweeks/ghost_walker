use crate::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Piece {
    pub value: u8,
    pub name: &'a str,
    pub team: Team,
}


impl Piece {
    pub fn new(value: u8, name: String, team: Team) -> Piece {
        Piece {
            value,
            name: &name,
            team,
        }
    }
}