#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Piece {
    White(Warrior, u8),
    Black(Warrior, u8),
    Empty,
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Warrior {
    Pawn,
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
}

impl Piece {
    pub fn is_black(&self) -> bool {
        match &self {
            Piece::Black(_, _) => return true,
            _ => return false,
        }
    }
    pub fn is_white(&self) -> bool {
        match &self {
            Piece::White(_, _) => return true,
            _ => return false,
        }
    }

    pub fn is_empty(&self) -> bool {
        match &self {
            Piece::Empty => return true,
            _ => return false,
        }
    }
}
