#[derive(Debug)]
struct Piece {
    value: u8,
    name: String,
    team: Team,
}


impl Piece {
    fn new(value: u8, name: String, team: Team) -> Piece {
        Piece {
            value,
            name,
            team,
        }
    }
}