// https://adventofcode.com/2022/day/6
use std::collections::HashSet;

fn parse(input: &str) -> Vec<char> {
    return input.chars().collect();
}

fn find_unique(parsed: &Vec<char>, length: usize) -> String {
    let mut set = HashSet::<char>::new();
    for (i, marker) in parsed.windows(length).enumerate() {
        if marker.iter().all(|&c| set.insert(c)) {
            return (i + length).to_string();
        }
        set.clear();
    }
    return 0.to_string();
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
