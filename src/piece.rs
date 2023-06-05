#[derive(Debug, Clone)]
pub enum Piece<'a> {
    White(&'a str, u8),
    Black(&'a str, u8),
    Empty
}
