use atoi::ascii_to_digit;
use itertools::Itertools;
use std::cmp::Ordering;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

fn map_hand_to_type(input: &str, jokers: bool) -> HandType {
    let mut cards: Vec<u8> = Vec::new();
    let mut amounts: Vec<u8> = Vec::new();
    let mut joker_amount: u8 = 0;
    for card in input.bytes() {
        if jokers && card == b'J' {
            joker_amount += 1;
        } else if let Some((i, _)) = cards.iter().find_position(|&&c| c == card) {
            amounts[i] += 1;
        } else {
            cards.push(card);
            amounts.push(1);
        }
    }

    let length = amounts.len();
    let max = amounts.into_iter().max().unwrap_or(0) + joker_amount;
    match max {
        5 => HandType::FiveOfAKind,
        4 => HandType::FourOfAKind,
        3 if length == 2 => HandType::FullHouse,
        3 => HandType::ThreeOfAKind,
        2 if length == 3 => HandType::TwoPair,
        2 => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

fn map_card_to_value(input: u8, jokers: bool) -> u8 {
    match input {
        b'A' => 14,
        b'K' => 13,
        b'Q' => 12,
        b'J' if jokers => 1,
        b'J' => 11,
        b'T' => 10,
        _ => ascii_to_digit(input).unwrap(),
    }
}

fn compare_hand((hand1, _): &(&str, u64), (hand2, _): &(&str, u64), jokers: bool) -> Ordering {
    let mut order = map_hand_to_type(hand1, jokers).cmp(&map_hand_to_type(hand2, jokers));
    let mut hands_bytes = hand1.bytes().zip(hand2.bytes());
    while order == Ordering::Equal
        && let Some((hand1_card, hand2_card)) = hands_bytes.next()
    {
        order = map_card_to_value(hand1_card, jokers).cmp(&map_card_to_value(hand2_card, jokers));
    }
    order
}

fn parse_hand_and_bid(input: &str) -> (&str, u64) {
    let (hand, bid_str) = input.split_once(' ').unwrap();
    let bid = bid_str.parse().unwrap();
    (hand, bid)
}

fn calculate_total_winnings(input: &str, jokers: bool) -> u64 {
    input
        .lines()
        .map(parse_hand_and_bid)
        .sorted_unstable_by(|hand1, hand2| compare_hand(hand1, hand2, jokers))
        .enumerate()
        .fold(0, |acc, (i, (_, bid))| acc + bid * (i as u64 + 1))
}

pub fn day07_star1(input: &str) -> u64 {
    calculate_total_winnings(input, false)
}

pub fn day07_star2(input: &str) -> u64 {
    calculate_total_winnings(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Ok, Result};
    use indoc::indoc;
    use pretty_assertions::assert_eq;
    use std::{fs::read_to_string, path::Path};

    const EXAMPLE_INPUT: &str = indoc! {"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483"};

    #[test]
    fn day07_star1_example() {
        let actual = day07_star1(EXAMPLE_INPUT);
        assert_eq!(actual, 6440);
    }

    #[test]
    fn day07_star1_final_answer() -> Result<()> {
        let file = read_to_string(Path::new("inputs/day07.txt"))?;
        let actual = day07_star1(&file);
        Ok(assert_eq!(actual, 246_163_188))
    }

    #[test]
    fn day07_star2_example() {
        let actual = day07_star2(EXAMPLE_INPUT);
        assert_eq!(actual, 5905);
    }

    #[test]
    fn day07_star2_final_answer() -> Result<()> {
        let file = read_to_string(Path::new("inputs/day07.txt"))?;
        let actual = day07_star2(&file);
        Ok(assert_eq!(actual, 245_794_069))
    }
}
