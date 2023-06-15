#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        return Point { x, y };
    }

    pub fn to_usize(&self) -> (usize, usize) {
        return (self.x as usize, self.y as usize);
    }

    pub fn is_off_board(&self) -> bool {
        let board = Point::new(8, 8);

        return self.x < 0 || self.x > board.x || self.y < 0 || self.y > board.y;
    }
}
