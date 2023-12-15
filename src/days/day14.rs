use crate::common::direction::Direction;
use itertools::Itertools;

fn iter_for_dir(length: usize, dir: Direction) -> Box<dyn Iterator<Item = usize>> {
    match dir {
        Direction::North | Direction::West => Box::new(0..length),
        Direction::South | Direction::East => Box::new((0..length).rev()),
    }
}

fn shift_rocks(state: &mut [u8], width: usize, dir: Direction) {
    for index in iter_for_dir(state.len(), dir) {
        if state[index] == b'O' {
            let mut space = index;
            match dir {
                Direction::North => {
                    while let Some(next_space) = space.checked_sub(width)
                        && state[next_space] == b'.'
                    {
                        space = next_space;
                    }
                }
                Direction::West => {
                    while let Some(next_space) = space.checked_sub(1)
                        && space % width != 0
                        && state[next_space] == b'.'
                    {
                        space = next_space;
                    }
                }
                Direction::South => {
                    while let next_space = space + width
                        && next_space < state.len()
                        && state[next_space] == b'.'
                    {
                        space = next_space;
                    }
                }
                Direction::East => {
                    while let next_space = space + 1
                        && space % width != width - 1
                        && state[next_space] == b'.'
                    {
                        space = next_space;
                    }
                }
            }
            state[index] = b'.';
            state[space] = b'O';
        }
    }
}

fn count_northern_support_load(state: &[u8], width: usize, height: usize) -> usize {
    let count_load = |(i, &byte)| {
        if byte == b'O' {
            height - i / width
        } else {
            0
        }
    };
    state.iter().enumerate().map(count_load).sum()
}

pub fn day14_star1(input: &str) -> usize {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let mut state = input.lines().join("").bytes().collect_vec();
    shift_rocks(&mut state, width, Direction::North);
    count_northern_support_load(&state, width, height)
}

pub fn day14_star2(input: &str) -> usize {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let mut state = input.lines().join("").bytes().collect_vec();
    let mut prev_states = vec![state.clone()];
    let mut cycle_start = 0;
    let mut cycle_length = 0;
    for i in 1..1000 {
        for dir in Direction::NWSE {
            shift_rocks(&mut state, width, dir);
        }
        if let Some(pos) = prev_states.iter().position(|prev| *prev == state) {
            cycle_start = pos;
            cycle_length = i - pos;
            break;
        }
        prev_states.push(state.clone());
    }
    let cycles_to_go = 1_000_000_000 - cycle_start;
    let final_index = cycle_start + (cycles_to_go % cycle_length);
    count_northern_support_load(&prev_states[final_index], width, height)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Ok, Result};
    use indoc::indoc;
    use pretty_assertions::assert_eq;
    use std::{fs::read_to_string, path::Path};

    const EXAMPLE_INPUT: &str = indoc! {"
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#...."};

    #[test]
    fn day14_star1_example() {
        let actual = day14_star1(EXAMPLE_INPUT);
        assert_eq!(actual, 136);
    }

    #[test]
    fn day14_star1_final_answer() -> Result<()> {
        let file = read_to_string(Path::new("inputs/day14.txt"))?;
        let actual = day14_star1(&file);
        Ok(assert_eq!(actual, 109_596))
    }

    #[test]
    fn day14_star2_example() {
        let actual = day14_star2(EXAMPLE_INPUT);
        assert_eq!(actual, 64);
    }

    #[test]
    fn day14_star2_final_answer() -> Result<()> {
        let file = read_to_string(Path::new("inputs/day14.txt"))?;
        let actual = day14_star2(&file);
        Ok(assert_eq!(actual, 96_105))
    }
}
