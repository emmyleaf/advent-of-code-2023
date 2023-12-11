use crate::common::parsing::parse_u64_vec;
use itertools::Itertools;
use rayon::prelude::*;
use std::{ops::Range, str::Lines};

#[derive(Debug)]
struct Mapping {
    dest_range: Range<u64>,
    source_range: Range<u64>,
}

#[derive(Debug)]
struct Map {
    mappings: Vec<Mapping>,
}

impl Map {
    pub fn transform(&self, mut number: u64) -> u64 {
        for mapping in &self.mappings {
            if mapping.source_range.contains(&number) {
                number = number + mapping.dest_range.start - mapping.source_range.start;
                break;
            }
        }
        number
    }

    pub fn transform_inverse(&self, mut number: u64) -> u64 {
        for m in &self.mappings {
            if m.dest_range.contains(&number) {
                number = number + m.source_range.start - m.dest_range.start;
                break;
            }
        }
        number
    }
}

fn parse_seeds_individual(lines: &mut Lines) -> Vec<u64> {
    parse_u64_vec(&lines.next().unwrap()[7..])
}

fn parse_seed_ranges(lines: &mut Lines) -> Vec<Range<u64>> {
    let seed_line = &lines.next().unwrap()[7..];
    let split = seed_line.split_ascii_whitespace();
    let tuples = split.map(str::parse).map(Result::unwrap).tuples();
    tuples
        .map(|(start, length)| (start..(start + length)))
        .collect()
}

fn parse_map(lines: &mut Lines) -> Map {
    let mut transforms = Vec::new();
    while let Some(line) = lines.next()
        && !line.is_empty()
    {
        let numbers = parse_u64_vec(line);
        transforms.push(Mapping {
            dest_range: numbers[0]..(numbers[0] + numbers[2]),
            source_range: numbers[1]..(numbers[1] + numbers[2]),
        });
    }
    Map {
        mappings: transforms,
    }
}

fn parse_all_maps(lines: &mut Lines) -> Vec<Map> {
    let mut maps = Vec::<Map>::new();
    while let Some(_) = lines.next() {
        // consume the map title line then parse the map
        maps.push(parse_map(lines));
    }
    maps
}

fn apply_maps(number: u64, maps: &[Map]) -> u64 {
    maps.iter().fold(number, |n, map| map.transform(n))
}

fn apply_maps_inverse(number: u64, maps: &[Map]) -> u64 {
    maps.iter().fold(number, |n, map| map.transform_inverse(n))
}

pub fn day05_star1(input: &str) -> u64 {
    let mut lines = input.lines();
    let seeds = parse_seeds_individual(&mut lines);
    lines.next(); // consume the first blank line
    let maps = parse_all_maps(&mut lines);

    seeds
        .into_par_iter()
        .map(|seed| apply_maps(seed, &maps))
        .min()
        .unwrap()
}

pub fn day05_star2(input: &str) -> u64 {
    let mut lines = input.lines();
    let seed_ranges = parse_seed_ranges(&mut lines);
    lines.next(); // consume the first blank line
    let mut maps = parse_all_maps(&mut lines);
    maps.reverse();

    // use the solution from star 1 as the upper bound!
    (0..910_845_529)
        .into_par_iter()
        .find_first(|&n| {
            let seed = apply_maps_inverse(n, &maps);
            seed_ranges.iter().any(|range| range.contains(&seed))
        })
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
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4"};

    #[test]
    fn day05_star1_example() {
        let actual = day05_star1(EXAMPLE_INPUT);
        assert_eq!(actual, 35);
    }

    #[test]
    fn day05_star1_final_answer() -> Result<()> {
        let file = read_to_string(Path::new("inputs/day05.txt"))?;
        let actual = day05_star1(&file);
        Ok(assert_eq!(actual, 910_845_529))
    }

    #[test]
    fn day05_star2_example() {
        let actual = day05_star2(EXAMPLE_INPUT);
        assert_eq!(actual, 46);
    }

    #[test]
    fn day05_star2_final_answer() -> Result<()> {
        let file = read_to_string(Path::new("inputs/day05.txt"))?;
        let actual = day05_star2(&file);
        Ok(assert_eq!(actual, 77_435_348))
    }
}
