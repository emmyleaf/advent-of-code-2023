use crate::common::direction::Direction;
use crate::common::grid2d::{Grid2D, Point2D};
use bit_vec::{BitBlock, BitVec};
use itertools::Itertools;

#[derive(Clone)]
struct State {
    pub grid: Grid2D,
    pub energised: BitVec,
    pub bounced_at: BitVec,
}

impl State {
    fn set_energised(&self, point: Point2D) {
        self.energised
            .set(point.1 * self.grid.width + point.0, true)
    }

    fn get_bounced_at(&self, point: Point2D) -> bool {
        self.bounced_at
            .get(point.1 * self.grid.width + point.0)
            .unwrap_or(false)
    }

    fn set_bounced_at(&self, point: Point2D) {
        self.bounced_at
            .set(point.1 * self.grid.width + point.0, true)
    }
}

impl std::ops::Add<State> for State {
    type Output = State;

    fn add(self, rhs: State) -> State {
        State {
            grid: self.grid,
            energised: (),
            bounced_at: (),
        }
    }
}

fn v_splitter(mut state: State, point: Point2D, dir: Direction) -> State {
    match dir {
        Direction::North | Direction::South => follow_path_rec(state, point, dir),
        Direction::East | Direction::West => {
            if todo!() {
                let north = follow_path_rec(state.clone(), point, Direction::North);
                let south = follow_path_rec(state, point, Direction::South);
                north + south
            }
            state
        }
    }
}

fn h_splitter(mut state: State, point: Point2D, dir: Direction) -> State {
    match dir {
        Direction::East | Direction::West => follow_path_rec(state, point, dir),
        Direction::North | Direction::South => {
            if todo!() {
                let east = follow_path_rec(state.clone(), point, Direction::East);
                let west = follow_path_rec(state, point, Direction::West);
                east + west
            }
            state
        }
    }
}

fn mirror_fs(mut state: State, point: Point2D, dir: Direction) -> State {
    let new_dir = match dir {
        Direction::North => Direction::East,
        Direction::East => Direction::North,
        Direction::South => Direction::West,
        Direction::West => Direction::South,
    };
    follow_path_rec(state, point, new_dir)
}

fn mirror_bs(mut state: State, point: Point2D, dir: Direction) -> State {
    let new_dir = match dir {
        Direction::North => Direction::West,
        Direction::East => Direction::South,
        Direction::South => Direction::East,
        Direction::West => Direction::North,
    };
    follow_path_rec(state, point, new_dir)
}

fn follow_path_rec(mut state: State, point: Point2D, dir: Direction) -> State {
    dbg!(&state.energised);
    let next_square = state.grid.move_from_point(point, dir);
    if let Some(next_point) = next_square {
        state.set_energised(next_point);
        match state.grid[next_point] {
            b'|' => v_splitter(state, next_point, dir),
            b'-' => h_splitter(state, next_point, dir),
            b'/' => mirror_fs(state, next_point, dir),
            b'\\' => mirror_bs(state, next_point, dir),
            _ => follow_path_rec(state, next_point, dir),
        }
    } else {
        state
    }
}

fn calculate_energised(grid: &Grid2D) -> usize {
    let mut unenergised = BitVec::from_elem(grid.data.len(), false);
    unenergised.set(0, true);
    let final_state = follow_path_rec(state, (0, 0), Direction::East);
    final_state
        .energised
        .blocks()
        .map(BitBlock::count_ones)
        .sum()
}

pub fn day16_star1(input: &str) -> usize {
    let lines = input.lines().collect_vec();
    calculate_energised(&Grid2D {
        width: lines[0].len(),
        height: lines.len(),
        data: lines.into_iter().flat_map(str::bytes).collect(),
    })
}

pub fn day16_star2(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Ok, Result};
    use indoc::indoc;
    use pretty_assertions::assert_eq;
    use std::{fs::read_to_string, path::Path};

    const EXAMPLE_INPUT: &str = indoc! {r"
    .|...\....
    |.-.\.....
    .....|-...
    ........|.
    ..........
    .........\
    ..../.\\..
    .-.-/..|..
    .|....-|.\
    ..//.|...."};

    #[test]
    fn day16_star1_example() {
        let actual = day16_star1(EXAMPLE_INPUT);
        assert_eq!(actual, 46);
    }

    #[test]
    fn day16_star1_final_answer() -> Result<()> {
        let file = read_to_string(Path::new("inputs/day16.txt"))?;
        let actual = day16_star1(&file);
        Ok(assert_eq!(actual, usize::MAX))
    }

    #[test]
    fn day16_star2_example() {
        let actual = day16_star2(EXAMPLE_INPUT);
        assert_eq!(actual, usize::MAX);
    }

    #[test]
    fn day16_star2_final_answer() -> Result<()> {
        let file = read_to_string(Path::new("inputs/day16.txt"))?;
        let actual = day16_star2(&file);
        Ok(assert_eq!(actual, usize::MAX))
    }
}
