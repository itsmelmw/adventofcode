// https://adventofcode.com/2022/day/6
use std::collections::HashSet;

use itertools::Itertools;

fn parse(input: &str) -> Vec<char> {
    return input.chars().collect();
}

fn find_unique(parsed: &Vec<char>, length: usize) -> String {
    let mut set = HashSet::<char>::new();
    let (i, _) = parsed
        .windows(length)
        .find_position(|marker| {
            set.clear();
            marker.iter().all(|&c| set.insert(c))
        })
        .unwrap();
    return (i + length).to_string();
}

fn solve1(parsed: &Vec<char>) -> String {
    return find_unique(parsed, 4);
}

fn solve2(parsed: &Vec<char>) -> String {
    return find_unique(parsed, 14);
}

pub fn solve(input: &str) -> (String, String) {
    let parsed = parse(input);
    return (solve1(&parsed), solve2(&parsed));
}
