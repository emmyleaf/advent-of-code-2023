use crate::common::maths::abs_diff;
use itertools::Itertools;

fn sum_paths_between_expanded_galaxies(input: &str, expansion: usize) -> usize {
    let width = input.lines().next().unwrap().len();
    let string = &input.lines().join("");
    let bytes = string.as_bytes();
    let length = bytes.len();

    let mut galaxies = Vec::new();
    let mut row_adj = 0;
    let mut empty_cols = Vec::<usize>::new();
    let mut row_empty = true;
    for i in 0..length {
        let byte = bytes[i];
        let x = i % width;
        let y = i / width;

        // Reset at start of row
        if x == 0 {
            row_empty = true;
        }

        // Handle galaxy!
        if byte == b'#' {
            let col_adj = expansion * empty_cols.iter().filter(|&&a| a < x).count();
            galaxies.push((x + col_adj, y + row_adj));
            row_empty = false;
        }

        // Adjust if needed at end of row
        if row_empty && x == width - 1 {
            row_adj += expansion;
        }

        // Check for empty columns while we traverse the first row
        if i < width {
            let mut x = i;
            let mut empty = true;
            while x < length {
                if bytes[x] != b'.' {
                    empty = false;
                    break;
                }
                x += width;
            }
            if empty {
                empty_cols.push(i);
            }
        }
    }

    galaxies
        .into_iter()
        .combinations(2)
        .map(|vec| abs_diff(vec[0].0, vec[1].0) + abs_diff(vec[0].1, vec[1].1))
        .sum()
}

pub fn day11_star1(input: &str) -> usize {
    sum_paths_between_expanded_galaxies(input, 1)
}

pub fn day11_star2(input: &str, expansion: usize) -> usize {
    sum_paths_between_expanded_galaxies(input, expansion - 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Ok, Result};
    use indoc::indoc;
    use pretty_assertions::assert_eq;
    use std::{fs::read_to_string, path::Path};

    const EXAMPLE_INPUT: &str = indoc! {"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#....."};

    #[test]
    fn day11_star1_example() {
        let actual = day11_star1(EXAMPLE_INPUT);
        assert_eq!(actual, 374);
    }

    #[test]
    fn day11_star1_final_answer() -> Result<()> {
        let file = read_to_string(Path::new("inputs/day11.txt"))?;
        let actual = day11_star1(&file);
        Ok(assert_eq!(actual, 9_233_514))
    }

    #[test]
    fn day11_star2_example() {
        let actual = day11_star2(EXAMPLE_INPUT, 100);
        assert_eq!(actual, 8410);
    }

    #[test]
    fn day11_star2_final_answer() -> Result<()> {
        let file = read_to_string(Path::new("inputs/day11.txt"))?;
        let actual = day11_star2(&file, 1_000_000);
        Ok(assert_eq!(actual, 363_293_506_944))
    }
}
