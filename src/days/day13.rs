use itertools::Itertools;

fn calculate_reflections(grid: &[&[u8]]) -> usize {
    let width = grid[0].len();
    let height = grid.len();

    // Check horizontal line reflection!
    for y in 0..(height - 1) {
        let mut rows_equal = true;
        for x in 0..width {
            if grid[y][x] != grid[y + 1][x] {
                rows_equal = false;
                break;
            }
        }
        if rows_equal {
            for offset in 1..=y {
                let lower = y - offset;
                let upper = y + 1 + offset;
                if upper >= height {
                    break;
                }
                for x in 0..width {
                    if grid[lower][x] != grid[upper][x] {
                        rows_equal = false;
                        break;
                    }
                }
            }
            if rows_equal {
                return 100 * (y + 1);
            }
        }
    }

    // Check vertical line reflection!
    for x in 0..(width - 1) {
        let mut columns_equal = true;
        for line in grid {
            if line[x] != line[x + 1] {
                columns_equal = false;
                break;
            }
        }
        if columns_equal {
            for offset in 1..=x {
                let left = x - offset;
                let right = x + 1 + offset;
                if right >= width {
                    break;
                }
                for line in grid {
                    if line[left] != line[right] {
                        columns_equal = false;
                        break;
                    }
                }
            }
            if columns_equal {
                return x + 1;
            }
        }
    }

    0
}

pub fn day13_star1(input: &str) -> usize {
    let lines = &input.lines().map(str::as_bytes).collect_vec();
    lines
        .split(|bytes| bytes.is_empty())
        .map(calculate_reflections)
        .sum()
}

pub fn day13_star2(input: &str) -> usize {
    let lines = &input.lines().map(str::as_bytes).collect_vec();
    lines
        .split(|bytes| bytes.is_empty())
        .map(calculate_reflections)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Ok, Result};
    use indoc::indoc;
    use pretty_assertions::assert_eq;
    use std::{fs::read_to_string, path::Path};

    const EXAMPLE_INPUT: &str = indoc! {"
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.
        
        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#"};

    #[test]
    fn day13_star1_example() {
        let actual = day13_star1(EXAMPLE_INPUT);
        assert_eq!(actual, 405);
    }

    #[test]
    fn day13_star1_final_answer() -> Result<()> {
        let file = read_to_string(Path::new("inputs/day13.txt"))?;
        let actual = day13_star1(&file);
        Ok(assert_eq!(actual, 35538))
    }

    #[test]
    fn day13_star2_example() {
        let actual = day13_star2(EXAMPLE_INPUT);
        assert_eq!(actual, 400);
    }

    #[test]
    fn day13_star2_final_answer() -> Result<()> {
        let file = read_to_string(Path::new("inputs/day13.txt"))?;
        let actual = day13_star2(&file);
        Ok(assert_eq!(actual, usize::MAX))
    }
}
