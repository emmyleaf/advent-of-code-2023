use super::direction::Direction;

pub type Point2D = (usize, usize);

#[derive(Clone)]
pub struct Grid2D {
    pub data: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl std::ops::Index<Point2D> for Grid2D {
    type Output = u8;
    fn index(&self, index: Point2D) -> &u8 {
        &self.data[index.1 * self.width + index.0]
    }
}

impl std::ops::IndexMut<Point2D> for Grid2D {
    fn index_mut(&mut self, index: Point2D) -> &mut u8 {
        &mut self.data[index.1 * self.width + index.0]
    }
}

impl Grid2D {
    pub const fn move_from_point(&self, point: Point2D, dir: Direction) -> Option<Point2D> {
        match dir {
            Direction::North if point.1 > 0 => Some((point.0, point.1 - 1)),
            Direction::West if point.0 > 0 => Some((point.0 - 1, point.1)),
            Direction::South if point.1 < self.height - 1 => Some((point.0, point.1 + 1)),
            Direction::East if point.0 < self.width - 1 => Some((point.0 + 1, point.1)),
            _ => None,
        }
    }
}
