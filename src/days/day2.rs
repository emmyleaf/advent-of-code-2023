use rayon::prelude::*;

fn is_colour_possible(num: u32, colour: &str) -> bool {
    match colour {
        "red" => num <= 12,
        "green" => num <= 13,
        "blue" => num <= 14,
        _ => false,
    }
}

fn is_set_possible(input: &str) -> bool {
    input.split(',').all(|string| {
        let space_split = string.trim_start().split_once(' ').unwrap();
        let num = space_split.0.parse::<u32>().unwrap();
        is_colour_possible(num, space_split.1)
    })
}

fn get_possible_game_id(input: &str) -> Option<u32> {
    let colon_split = input.split_once(':').unwrap();
    let game_id = colon_split.0[5..].parse::<u32>().unwrap();
    let mut semicolon_split = colon_split.1.split(';');
    semicolon_split.all(is_set_possible).then_some(game_id)
}

fn calculate_game_power(input: &str) -> u32 {
    let colour_sets = input.split_once(':').unwrap().1.split([';', ',']);
    let mut rgb: (u32, u32, u32) = (0, 0, 0);
    for colour_set in colour_sets {
        if let Some((num, colour)) = colour_set.trim_start().split_once(' ') {
            let n = num.parse::<u32>().unwrap();
            match colour {
                "red" if n > rgb.0 => rgb.0 = n,
                "green" if n > rgb.1 => rgb.1 = n,
                "blue" if n > rgb.2 => rgb.2 = n,
                _ => (),
            }
        }
    }
    rgb.0 * rgb.1 * rgb.2
}

pub fn day2_star1(input: &str) -> u32 {
    input.par_lines().filter_map(get_possible_game_id).sum()
}

pub fn day2_star2(input: &str) -> u32 {
    input.par_lines().map(calculate_game_power).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Ok, Result};
    use indoc::indoc;
    use pretty_assertions::assert_eq;
    use std::{fs::read_to_string, path::Path};

    const EXAMPLE_INPUT: &str = indoc! {"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"};

    #[test]
    fn day2_star1_example() {
        let actual = day2_star1(EXAMPLE_INPUT);
        assert_eq!(actual, 8);
    }

    #[test]
    fn day2_star1_final_answer() -> Result<()> {
        let file = read_to_string(Path::new("inputs/day2.txt"))?;
        let actual = day2_star1(&file);
        Ok(assert_eq!(actual, 2449))
    }

    #[test]
    fn day2_star2_example() {
        let actual = day2_star2(EXAMPLE_INPUT);
        assert_eq!(actual, 2286);
    }

    #[test]
    fn day2_star2_final_answer() -> Result<()> {
        let file = read_to_string(Path::new("inputs/day2.txt"))?;
        let actual = day2_star2(&file);
        Ok(assert_eq!(actual, 63981))
    }
}
