use std::collections::HashSet;

pub fn parse_u32_set(input: &str) -> HashSet<u32> {
    let split = input.split_ascii_whitespace();
    split.map(str::parse).map(Result::unwrap).collect()
}

pub fn parse_u64_vec(input: &str) -> Vec<u64> {
    let split = input.split_ascii_whitespace();
    split.map(str::parse).map(Result::unwrap).collect()
}

pub fn parse_i64_vec(input: &str) -> Vec<i64> {
    let split = input.split_ascii_whitespace();
    split.map(str::parse).map(Result::unwrap).collect()
}

pub fn parse_f64_iter(input: &str) -> impl Iterator<Item = f64> + '_ {
    let split = input.split_ascii_whitespace();
    split.map(str::parse).map(Result::unwrap)
}

pub fn parse_f64_with_spaces(input: &str) -> f64 {
    input.replace(' ', "").parse().unwrap()
}
