use rayon::prelude::*;

const MAXIMUM_RED: u32 = 12;
const MAXIMUM_GREEN: u32 = 13;
const MAXIMUM_BLUE: u32 = 14;

fn is_colour_possible(num: u32, colour: &str) -> bool {
    match colour {
        "red" => num <= MAXIMUM_RED,
        "green" => num <= MAXIMUM_GREEN,
        "blue" => num <= MAXIMUM_BLUE,
        _ => false,
    }
}

fn is_set_possible(input: &str) -> bool {
    input.split(',').all(|string| {
        let mut space_split = string.trim_start().split(' ');
        let num = space_split.next().unwrap().parse::<u32>().unwrap();
        is_colour_possible(num, space_split.next().unwrap())
    })
}

fn get_possible_game_id(input: &str) -> Option<u32> {
    let mut colon_split = input.split(':');
    let game_id = colon_split.next()?[5..].parse::<u32>().unwrap();
    let mut semicolon_split = colon_split.next()?.split(';');
    semicolon_split.all(is_set_possible).then_some(game_id)
}

pub fn day2_star1(input: &str) -> u32 {
    input
        .split('\n')
        .par_bridge()
        .filter_map(get_possible_game_id)
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
    fn day2_star1_example() {
        let input = indoc! {"
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"};
        let actual = day2_star1(input);
        assert_eq!(actual, 8);
    }

    #[test]
    fn day2_star1_final_answer() -> Result<()> {
        let file = read_to_string(Path::new("inputs/day2.txt"))?;
        let actual = day2_star1(&file);
        Ok(assert_eq!(actual, 2449))
    }
}
