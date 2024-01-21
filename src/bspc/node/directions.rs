pub enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    pub fn get_string(&self) -> String {
        String::from(match self {
            Direction::North => "north",
            Direction::East => "east",
            Direction::South => "south",
            Direction::West => "west"
        })
    }
}
