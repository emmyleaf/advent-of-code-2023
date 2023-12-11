use crate::common::parsing::parse_i64_vec;
use itertools::Itertools;
use rayon::prelude::*;

fn calculate_next_in_sequence(sequence: &[i64]) -> i64 {
    let differences: Vec<i64> = sequence.iter().map_windows(|[&a, &b]| b - a).collect();
    if differences.iter().all_equal() {
        sequence.last().unwrap() + differences[0]
    } else {
        sequence.last().unwrap() + calculate_next_in_sequence(&differences)
    }
}

fn calculate_prev_in_sequence(sequence: &[i64]) -> i64 {
    let differences: Vec<i64> = sequence.iter().map_windows(|[&a, &b]| b - a).collect();
    if differences.iter().all_equal() {
        sequence.first().unwrap() - differences[0]
    } else {
        sequence.first().unwrap() - calculate_prev_in_sequence(&differences)
    }
}

pub fn day09_star1(input: &str) -> i64 {
    input
        .par_lines()
        .map(parse_i64_vec)
        .map(|seq| calculate_next_in_sequence(&seq))
        .sum()
}

pub fn day09_star2(input: &str) -> i64 {
    input
        .par_lines()
        .map(parse_i64_vec)
        .map(|seq| calculate_prev_in_sequence(&seq))
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
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45"};

    #[test]
    fn day09_star1_example() {
        let actual = day09_star1(EXAMPLE_INPUT);
        assert_eq!(actual, 114);
    }

    #[test]
    fn day09_star1_final_answer() -> Result<()> {
        let file = read_to_string(Path::new("inputs/day09.txt"))?;
        let actual = day09_star1(&file);
        Ok(assert_eq!(actual, 1_887_980_197))
    }

    #[test]
    fn day09_star2_example() {
        let actual = day09_star2(EXAMPLE_INPUT);
        assert_eq!(actual, 2);
    }

    #[test]
    fn day09_star2_final_answer() -> Result<()> {
        let file = read_to_string(Path::new("inputs/day09.txt"))?;
        let actual = day09_star2(&file);
        Ok(assert_eq!(actual, 990))
    }
}
