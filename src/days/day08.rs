use std::collections::HashMap;

fn parse_input(input: &str) -> (&str, HashMap<&str, (&str, &str)>) {
    let mut lines = input.lines();
    let choices = lines.next().unwrap();
    lines.next(); // burn the empty line
    let nodes = lines
        .map(|line| (&line[..3], (&line[7..10], &line[12..15])))
        .collect();
    (choices, nodes)
}

fn count_steps(choices: &str, nodes: &HashMap<&str, (&str, &str)>, starting_node: &str) -> u64 {
    let mut current_node = starting_node;
    let mut steps = 0;
    let mut choices_iter = choices.bytes().cycle();
    while !current_node.ends_with('Z') {
        let choice = choices_iter.next().unwrap();
        let options = nodes[current_node];
        current_node = if choice == b'L' { options.0 } else { options.1 };
        steps += 1;
    }
    steps as u64
}

pub fn day08_star1(input: &str) -> u64 {
    let (choices, nodes) = parse_input(input);
    count_steps(choices, &nodes, "AAA")
}

pub fn day08_star2(input: &str) -> u64 {
    let (choices, nodes) = parse_input(input);
    nodes
        .keys()
        .filter(|&&node| node.ends_with('A'))
        .map(|&node| count_steps(choices, &nodes, node))
        .reduce(num::integer::lcm)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Ok, Result};
    use indoc::indoc;
    use pretty_assertions::assert_eq;
    use std::{fs::read_to_string, path::Path};

    const EXAMPLE_INPUT: &str = indoc! {"
        RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)"};

    #[test]
    fn day08_star1_example1() {
        let actual = day08_star1(EXAMPLE_INPUT);
        assert_eq!(actual, 2);
    }

    #[test]
    fn day08_star1_example2() {
        let input = indoc! {"
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)"};
        let actual = day08_star1(input);
        assert_eq!(actual, 6);
    }

    #[test]
    fn day08_star1_final_answer() -> Result<()> {
        let file = read_to_string(Path::new("inputs/day08.txt"))?;
        let actual = day08_star1(&file);
        Ok(assert_eq!(actual, 13301))
    }

    #[test]
    fn day08_star2_example1() {
        let actual = day08_star2(EXAMPLE_INPUT);
        assert_eq!(actual, 2);
    }

    #[test]
    fn day08_star2_example2() {
        let input = indoc! {"
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)"};
        let actual = day08_star2(input);
        assert_eq!(actual, 6);
    }

    #[test]
    fn day08_star2_final_answer() -> Result<()> {
        let file = read_to_string(Path::new("inputs/day08.txt"))?;
        let actual = day08_star2(&file);
        Ok(assert_eq!(actual, 7_309_459_565_207))
    }
}
