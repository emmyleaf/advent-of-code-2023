use rayon::prelude::*;
use std::collections::HashSet;

fn parse_number_set(input: &str) -> HashSet<u32> {
    input
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn calculate_card_score(input: &str) -> u32 {
    let mut parts = input.split([':', '|']);
    let winning_numbers = parse_number_set(parts.nth(1).unwrap());
    let card_numbers = parse_number_set(parts.next().unwrap());

    card_numbers
        .into_iter()
        .filter(|x| winning_numbers.contains(x))
        .fold(0, |acc, _| match acc {
            0 => 1,
            _ => acc * 2,
        })
}

pub fn day4_star1(input: &str) -> u32 {
    input
        .split('\n')
        .par_bridge()
        .map(calculate_card_score)
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
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"};

    #[test]
    fn day4_star1_example() {
        let actual = day4_star1(EXAMPLE_INPUT);
        assert_eq!(actual, 13);
    }

    #[test]
    fn day4_star1_final_answer() -> Result<()> {
        let file = read_to_string(Path::new("inputs/day4.txt"))?;
        let actual = day4_star1(&file);
        Ok(assert_eq!(actual, 21213))
    }
}
