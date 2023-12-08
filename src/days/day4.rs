use crate::common::parsing::parse_u32_set;
use rayon::prelude::*;
use std::collections::HashSet;

fn parse_number_sets(input: &str) -> (HashSet<u32>, HashSet<u32>) {
    let mut parts = input.split([':', '|']);
    let winning_numbers = parse_u32_set(parts.nth(1).unwrap());
    let card_numbers = parse_u32_set(parts.next().unwrap());
    (winning_numbers, card_numbers)
}

fn calculate_card_score(input: &str) -> u32 {
    let (winning_numbers, card_numbers) = parse_number_sets(input);
    card_numbers
        .into_iter()
        .filter(|x| winning_numbers.contains(x))
        .fold(0, |acc, _| match acc {
            0 => 1,
            _ => acc * 2,
        })
}

fn count_win_amount(input: &str) -> usize {
    let (winning_numbers, card_numbers) = parse_number_sets(input);
    card_numbers
        .into_iter()
        .filter(|x| winning_numbers.contains(x))
        .count()
}

pub fn day4_star1(input: &str) -> u32 {
    input.par_lines().map(calculate_card_score).sum()
}

pub fn day4_star2(input: &str) -> u32 {
    let cards: Vec<&str> = input.lines().collect();
    let mut card_amounts: Vec<u32> = vec![1; cards.len()];
    for (i, card) in cards.iter().enumerate() {
        let win_amount = count_win_amount(card);
        let added_amount = card_amounts[i];
        for card_amount in card_amounts.iter_mut().take(i + win_amount + 1).skip(i + 1) {
            *card_amount += added_amount;
        }
    }
    card_amounts.into_iter().sum()
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

    #[test]
    fn day4_star2_example() {
        let actual = day4_star2(EXAMPLE_INPUT);
        assert_eq!(actual, 30);
    }

    #[test]
    fn day4_star2_final_answer() -> Result<()> {
        let file = read_to_string(Path::new("inputs/day4.txt"))?;
        let actual = day4_star2(&file);
        Ok(assert_eq!(actual, 8_549_735))
    }
}
