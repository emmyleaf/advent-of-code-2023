use itertools::Itertools;
use lazy_static::lazy_static;
use rayon::prelude::*;
use std::{collections::HashMap, iter::repeat, sync::RwLock};

type Cache = HashMap<(Vec<u8>, Vec<usize>), usize>;

lazy_static! {
    static ref CACHE: RwLock<Cache> = RwLock::new(Cache::new());
}

fn count_arrangements_rec(records: &[u8], groups: &[usize]) -> usize {
    // Check the cache for this pattern
    if let Ok(cache) = CACHE.read()
        && let Some(&count) = cache.get(&(records.to_vec(), groups.to_vec()))
    {
        return count;
    }

    // Recurse and cache!
    let count = match records.first() {
        Some(b'.') => {
            // If we have no damage here, move on to the next spring
            count_arrangements_rec(&records[1..], groups)
        }
        Some(b'#') => {
            // If we have damage here, figure out where we are in relation to the groups
            count_damage_arrangements_rec(records, groups)
        }
        Some(b'?') => {
            // If we have unknown condition, calculate both possibilities
            count_arrangements_rec(&records[1..], groups)
                + count_damage_arrangements_rec(records, groups)
        }
        None => {
            // If we have used up all records & groups, we have a valid arrangement!
            usize::from(groups.is_empty())
        }
        Some(_) => panic!("this shouldn't happen :):):)"),
    };
    if let Ok(mut cache) = CACHE.write() {
        cache.insert((records.to_vec(), groups.to_vec()), count);
    }
    count
}

fn count_damage_arrangements_rec(records: &[u8], groups: &[usize]) -> usize {
    // No groups left to match
    if groups.is_empty() {
        return 0;
    }
    let group_length = groups[0];

    // Not enough chars to match this group
    if records.len() < group_length {
        return 0;
    }

    // We find a definitely undamaged spring in the group
    if records.iter().take(group_length).contains(&b'.') {
        return 0;
    }

    // We have the same length as the group
    if records.len() == group_length {
        // If this is the last group, we have a valid arrangement!
        return usize::from(groups.len() == 1);
    }

    // The next spring after a matched group is also damaged
    if records[group_length] == b'#' {
        return 0;
    }

    // Recurse!
    count_arrangements_rec(&records[group_length + 1..], &groups[1..])
}

fn count_arrangements(input: &str) -> usize {
    let (record_str, group_str) = input.split_once(' ').unwrap();
    let groups: Vec<usize> = group_str.split(',').map(|x| x.parse().unwrap()).collect();
    count_arrangements_rec(record_str.as_bytes(), &groups)
}

fn unfold_and_count_arrangements(input: &str) -> usize {
    let (record_str, group_str) = input.split_once(' ').unwrap();
    let unf_record_str = repeat(record_str).take(5).join("?");
    let unf_groups = repeat(group_str).take(5).join(",");
    let groups: Vec<usize> = unf_groups.split(',').map(|x| x.parse().unwrap()).collect();
    count_arrangements_rec(unf_record_str.as_bytes(), &groups)
}

pub fn day12_star1(input: &str) -> usize {
    input.par_lines().map(count_arrangements).sum()
}

pub fn day12_star2(input: &str) -> usize {
    input.par_lines().map(unfold_and_count_arrangements).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Ok, Result};
    use indoc::indoc;
    use pretty_assertions::assert_eq;
    use std::{fs::read_to_string, path::Path};

    const EXAMPLE_INPUT: &str = indoc! {"
        ???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1"};

    #[test]
    fn day12_star1_example() {
        let actual = day12_star1(EXAMPLE_INPUT);
        assert_eq!(actual, 21);
    }

    #[test]
    fn day12_star1_final_answer() -> Result<()> {
        let file = read_to_string(Path::new("inputs/day12.txt"))?;
        let actual = day12_star1(&file);
        Ok(assert_eq!(actual, 7221))
    }

    #[test]
    fn day12_star2_example() {
        let actual = day12_star2(EXAMPLE_INPUT);
        assert_eq!(actual, 525_152);
    }

    #[test]
    fn day12_star2_final_answer() -> Result<()> {
        let file = read_to_string(Path::new("inputs/day12.txt"))?;
        let actual = day12_star2(&file);
        Ok(assert_eq!(actual, 7_139_671_893_722))
    }
}
