use crate::common::parsing::{parse_f64_iter, parse_f64_with_spaces};

fn calculate_winning_amount((time, distance): (f64, f64)) -> u64 {
    // calculate the difference between the zeroes of the quadratic equation:
    // x^2 - time * x + distance = 0
    let disc_sqrt = time.mul_add(time, -(4.0 * distance)).sqrt();
    let root1 = ((time - disc_sqrt) / 2.0).floor();
    let root2 = ((time + disc_sqrt) / 2.0).ceil();
    (root2 - root1) as u64 - 1
}

pub fn day06_star1(input: &str) -> u64 {
    let mut lines = input.lines();
    let times = parse_f64_iter(&lines.next().unwrap()[11..]);
    let distances = parse_f64_iter(&lines.next().unwrap()[11..]);
    times.zip(distances).map(calculate_winning_amount).product()
}

pub fn day06_star2(input: &str) -> u64 {
    let mut lines = input.lines();
    let time = parse_f64_with_spaces(&lines.next().unwrap()[11..]);
    let distance = parse_f64_with_spaces(&lines.next().unwrap()[11..]);
    calculate_winning_amount((time, distance))
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Ok, Result};
    use indoc::indoc;
    use pretty_assertions::assert_eq;
    use std::{fs::read_to_string, path::Path};

    const EXAMPLE_INPUT: &str = indoc! {"
        Time:      7  15   30
        Distance:  9  40  200"};

    #[test]
    fn day06_star1_example() {
        let actual = day06_star1(EXAMPLE_INPUT);
        assert_eq!(actual, 288);
    }

    #[test]
    fn day06_star1_final_answer() -> Result<()> {
        let file = read_to_string(Path::new("inputs/day06.txt"))?;
        let actual = day06_star1(&file);
        Ok(assert_eq!(actual, 2_065_338))
    }

    #[test]
    fn day06_star2_example() {
        let actual = day06_star2(EXAMPLE_INPUT);
        assert_eq!(actual, 71503);
    }

    #[test]
    fn day06_star2_final_answer() -> Result<()> {
        let file = read_to_string(Path::new("inputs/day06.txt"))?;
        let actual = day06_star2(&file);
        Ok(assert_eq!(actual, 34_934_171))
    }
}
