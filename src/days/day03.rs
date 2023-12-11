const fn is_ascii_symbol_not_dot(char: u8) -> bool {
    char != b'.' && char.is_ascii_punctuation()
}

fn find_engine_part_sum(bytes: &[u8]) -> u32 {
    let width = bytes.iter().position(|c| *c == b'\n').unwrap() + 1;
    let mut part_sum: u32 = 0;
    let mut index = 0;
    while index < bytes.len() {
        if is_ascii_symbol_not_dot(bytes[index]) {
            let x = index % width;
            let y = index / width;
            // check adjacent spaces for numbers
            let mut num_y = y - 1;
            while num_y <= y + 1 {
                let mut num_x = x - 1;
                while num_x <= x + 1 {
                    if bytes[num_y * width + num_x].is_ascii_digit() {
                        // find the start of the number
                        while num_x > 0 && bytes[num_y * width + num_x - 1].is_ascii_digit() {
                            num_x -= 1;
                        }
                        let num_start = num_y * width + num_x;
                        while bytes[num_y * width + num_x].is_ascii_digit() {
                            num_x += 1;
                        }
                        let num_bytes = &bytes[num_start..(num_y * width + num_x)];
                        part_sum += atoi::atoi::<u32>(num_bytes).unwrap();
                    }
                    num_x += 1;
                }
                num_y += 1;
            }
        }
        index += 1;
    }
    part_sum
}

fn find_gear_ratio_sum(bytes: &[u8]) -> u32 {
    let width = bytes.iter().position(|c| *c == b'\n').unwrap() + 1;
    let mut ratio_sum: u32 = 0;
    let mut index = 0;
    while index < bytes.len() {
        if is_ascii_symbol_not_dot(bytes[index]) {
            let x = index % width;
            let y = index / width;
            let mut first_part_number = 0;
            // check adjacent spaces for numbers
            let mut num_y = y - 1;
            while num_y <= y + 1 {
                let mut num_x = x - 1;
                while num_x <= x + 1 {
                    if bytes[num_y * width + num_x].is_ascii_digit() {
                        // find the start of the number
                        while num_x > 0 && bytes[num_y * width + num_x - 1].is_ascii_digit() {
                            num_x -= 1;
                        }
                        let num_start = num_y * width + num_x;
                        while bytes[num_y * width + num_x].is_ascii_digit() {
                            num_x += 1;
                        }
                        let num_bytes = &bytes[num_start..(num_y * width + num_x)];
                        let part_number = atoi::atoi::<u32>(num_bytes).unwrap();
                        if first_part_number == 0 {
                            first_part_number = part_number;
                        } else {
                            ratio_sum += first_part_number * part_number;
                        }
                    }
                    num_x += 1;
                }
                num_y += 1;
            }
        }
        index += 1;
    }
    ratio_sum
}

pub fn day03_star1(input: &str) -> u32 {
    find_engine_part_sum(&input.bytes().collect::<Vec<u8>>())
}

pub fn day03_star2(input: &str) -> u32 {
    find_gear_ratio_sum(&input.bytes().collect::<Vec<u8>>())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Ok, Result};
    use indoc::indoc;
    use pretty_assertions::assert_eq;
    use std::{fs::read_to_string, path::Path};

    const EXAMPLE_INPUT: &str = indoc! {"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598.."};

    #[test]
    fn day03_star1_example() {
        let actual = day03_star1(EXAMPLE_INPUT);
        assert_eq!(actual, 4361);
    }

    #[test]
    fn day03_star1_final_answer() -> Result<()> {
        let file = read_to_string(Path::new("inputs/day03.txt"))?;
        let actual = day03_star1(&file);
        Ok(assert_eq!(actual, 553_079))
    }

    #[test]
    fn day03_star2_example() {
        let actual = day03_star2(EXAMPLE_INPUT);
        assert_eq!(actual, 467_835);
    }

    #[test]
    fn day03_star2_final_answer() -> Result<()> {
        let file = read_to_string(Path::new("inputs/day03.txt"))?;
        let actual = day03_star2(&file);
        Ok(assert_eq!(actual, 84_363_105))
    }
}
