#[derive(Debug, Clone, PartialEq)]
pub enum Piece<'a> {
    White(&'a str, u8),
    Black(&'a str, u8),
    Empty,
}

impl Piece<'_> {
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
