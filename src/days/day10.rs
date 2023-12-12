use itertools::Itertools;

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    const VALUES: [Self; 4] = [Self::North, Self::East, Self::South, Self::West];
}

const NORTH_CONNECTIONS: [u8; 3] = [b'|', b'7', b'F'];
const EAST_CONNECTIONS: [u8; 3] = [b'-', b'7', b'J'];
const SOUTH_CONNECTIONS: [u8; 3] = [b'|', b'L', b'J'];
const WEST_CONNECTIONS: [u8; 3] = [b'-', b'L', b'F'];

fn trace_path(bytes: &[u8], width: usize) -> (usize, Vec<usize>) {
    let len = bytes.len();

    // Find starting position and one connection to begin following
    let (start, _) = bytes.iter().find_position(|&&b| b == b'S').unwrap();
    let start_dir = Direction::VALUES.into_iter().find(|&dir| match dir {
        Direction::North => {
            start > width && NORTH_CONNECTIONS.iter().any(|&p| p == bytes[start - width])
        }
        Direction::East => start < len && EAST_CONNECTIONS.iter().any(|&p| p == bytes[start + 1]),
        Direction::South => {
            start < len - width + 1 && SOUTH_CONNECTIONS.iter().any(|&p| p == bytes[start + width])
        }
        Direction::West => start > 0 && WEST_CONNECTIONS.iter().any(|&p| p == bytes[start - 1]),
    });

    // Start following this path until we reach the start again!
    let mut steps = 0;
    let mut index = start;
    let mut direction = start_dir.unwrap();
    let mut path = vec![index];
    loop {
        steps += 1;
        match direction {
            Direction::North => {
                index -= width;
                direction = match bytes[index] {
                    b'|' => Direction::North,
                    b'7' => Direction::West,
                    b'F' => Direction::East,
                    _ => break,
                };
            }
            Direction::East => {
                index += 1;
                direction = match bytes[index] {
                    b'-' => Direction::East,
                    b'7' => Direction::South,
                    b'J' => Direction::North,
                    _ => break,
                };
            }
            Direction::South => {
                index += width;
                direction = match bytes[index] {
                    b'|' => Direction::South,
                    b'L' => Direction::East,
                    b'J' => Direction::West,
                    _ => break,
                };
            }
            Direction::West => {
                index -= 1;
                direction = match bytes[index] {
                    b'-' => Direction::West,
                    b'L' => Direction::North,
                    b'F' => Direction::South,
                    _ => break,
                };
            }
        }
        path.push(index);
    }

    (steps, path)
}

pub fn day10_star1(input: &str) -> usize {
    let width = input.lines().next().unwrap().len();
    let string = input.lines().join("");
    let bytes = string.as_bytes();
    let (steps, _) = trace_path(bytes, width);
    steps / 2
}

pub fn day10_star2(input: &str) -> usize {
    let width = input.lines().next().unwrap().len();
    let string = input.lines().join("");
    let bytes = string.as_bytes();
    let (_, path) = trace_path(bytes, width);

    let mut squares_inside = 0;
    let mut inside = false;
    for (index, byte) in bytes.iter().enumerate() {
        if index % width == 0 {
            inside = false;
        }
        let on_path = path.contains(&index);
        // Flip on starting point, vertical lines, and the start of S-bends
        if [b'S', b'|', b'F', b'7'].contains(byte) && on_path {
            inside = !inside;
        }
        if inside && !on_path {
            squares_inside += 1;
        }
    }
    squares_inside
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Ok, Result};
    use indoc::indoc;
    use pretty_assertions::assert_eq;
    use std::{fs::read_to_string, path::Path};

    #[test]
    fn day10_star1_example1() {
        let input = indoc! {"
            -L|F7
            7S-7|
            L|7||
            -L-J|
            L|-JF"};
        let actual = day10_star1(input);
        assert_eq!(actual, 4);
    }

    #[test]
    fn day10_star1_example2() {
        let input = indoc! {"
            ..F7.
            .FJ|.
            SJ.L7
            |F--J
            LJ..."};
        let actual = day10_star1(input);
        assert_eq!(actual, 8);
    }

    #[test]
    fn day10_star1_final_answer() -> Result<()> {
        let file = read_to_string(Path::new("inputs/day10.txt"))?;
        let actual = day10_star1(&file);
        Ok(assert_eq!(actual, 6842))
    }

    #[test]
    fn day10_star2_example1() {
        let input = indoc! {"
            .F----7F7F7F7F-7....
            .|F--7||||||||FJ....
            .||.FJ||||||||L7....
            FJL7L7LJLJ||LJ.L-7..
            L--J.L7...LJS7F-7L7.
            ....F-J..F7FJ|L7L7L7
            ....L7.F7||L7|.L7L7|
            .....|FJLJ|FJ|F7|.LJ
            ....FJL-7.||.||||...
            ....L---J.LJ.LJLJ..."};
        let actual = day10_star2(input);
        assert_eq!(actual, 8);
    }

    #[test]
    fn day10_star2_example2() {
        let input = indoc! {"
            FF7FSF7F7F7F7F7F---7
            L|LJ||||||||||||F--J
            FL-7LJLJ||||||LJL-77
            F--JF--7||LJLJ7F7FJ-
            L---JF-JLJ.||-FJLJJ7
            |F|F-JF---7F7-L7L|7|
            |FFJF7L7F-JF7|JL---7
            7-L-JL7||F7|L7F-7F7|
            L.L7LFJ|||||FJL7||LJ
            L7JLJL-JLJLJL--JLJ.L"};
        let actual = day10_star2(input);
        assert_eq!(actual, 10);
    }

    #[test]
    fn day10_star2_final_answer() -> Result<()> {
        let file = read_to_string(Path::new("inputs/day10.txt"))?;
        let actual = day10_star2(&file);
        Ok(assert_eq!(actual, 393))
    }
}
