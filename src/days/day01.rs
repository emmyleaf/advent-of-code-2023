use aho_corasick::{AhoCorasick, Match};
use lazy_static::lazy_static;
use rayon::prelude::*;

const WORD_AND_DIGIT_PATTERNS: &[&str] = &[
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1", "2",
    "3", "4", "5", "6", "7", "8", "9",
];

lazy_static! {
    static ref AHO_CORASICK: AhoCorasick = AhoCorasick::new(WORD_AND_DIGIT_PATTERNS).unwrap();
}

fn parse_first_digit<I: Iterator<Item = char>>(mut chars: I) -> u32 {
    chars
        .find(char::is_ascii_digit)
        .and_then(|c| c.to_digit(10))
        .unwrap()
}

fn parse_calibration_line_digits_only(input: &str) -> u32 {
    let first_digit = parse_first_digit(input.chars());
    let last_digit = parse_first_digit(input.chars().rev());
    first_digit * 10 + last_digit
}

fn match_to_digit(the_match: Match) -> Option<u32> {
    let string = WORD_AND_DIGIT_PATTERNS[the_match.pattern()];
    match string {
        "zero" => Some(0),
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        x if let c = x.chars().next().unwrap()
            && c.is_ascii_digit() =>
        {
            c.to_digit(10)
        }
        _ => None,
    }
}

fn parse_calibration_line_words_and_digits(input: &str) -> u32 {
    let ac = AHO_CORASICK.clone();
    let mut matches = ac.find_overlapping_iter(input);
    let first_digit = matches.next().and_then(match_to_digit).unwrap();
    let last_digit = matches
        .last()
        .and_then(match_to_digit)
        .unwrap_or(first_digit);
    first_digit * 10 + last_digit
}

pub fn day01_star1(input: &str) -> u32 {
    input
        .par_split_whitespace()
        .map(parse_calibration_line_digits_only)
        .sum()
}

pub fn day01_star2(input: &str) -> u32 {
    input
        .par_split_whitespace()
        .map(parse_calibration_line_words_and_digits)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Ok, Result};
    use indoc::indoc;
    use pretty_assertions::assert_eq;
    use std::{fs::read_to_string, path::Path};

    #[test]
    fn day01_star1_example() {
        let input = indoc! {"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet"};
        let actual = day01_star1(input);
        assert_eq!(actual, 142);
    }

    #[test]
    fn day01_star1_final_answer() -> Result<()> {
        let file = read_to_string(Path::new("inputs/day01.txt"))?;
        let actual = day01_star1(&file);
        Ok(assert_eq!(actual, 54239))
    }

    #[test]
    fn day01_star2_example() {
        let input = indoc! {"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen"};
        let actual = day01_star2(input);
        assert_eq!(actual, 281);
    }

    #[test]
    fn day01_star2_final_answer() -> Result<()> {
        let file = read_to_string(Path::new("inputs/day01.txt"))?;
        let actual = day01_star2(&file);
        Ok(assert_eq!(actual, 55343))
    }
}
