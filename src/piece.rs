#[derive(Debug, Clone)]
pub enum Piece<'a> {
    King(&'a str, u8),
    Queen(&'a str, u8),
    Pawn(&'a str, u8),
    Rook(&'a str, u8),
    Knight(&'a str, u8),
    Bishop(&'a str, u8),
}
