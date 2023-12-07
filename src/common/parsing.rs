use std::collections::HashSet;

pub fn parse_u32_set(input: &str) -> HashSet<u32> {
    let split = input.split_ascii_whitespace();
    split.map(str::parse).map(Result::unwrap).collect()
}

pub fn parse_u64_vec(input: &str) -> Vec<u64> {
    let split = input.split_ascii_whitespace();
    split.map(str::parse).map(Result::unwrap).collect()
}
