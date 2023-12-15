#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub const NWSE: [Self; 4] = [Self::North, Self::West, Self::South, Self::East];
}
