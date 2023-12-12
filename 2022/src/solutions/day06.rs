// https://adventofcode.com/2022/day/6

use aoc_utils::solutions::{InputDir, Part, Solution};
use itertools::Itertools;
use std::collections::HashSet;

pub struct Day06 {
    data: Vec<char>,
}

impl Solution for Day06 {
    fn title(&self) -> &str {
        "Tuning Trouble"
    }
    fn parse(input: &str) -> Self {
        let data = input.chars().collect();
        Self { data }
    }
    fn solve_part_1(&self) -> String {
        self.find_unique(4)
    }
    fn solve_part_2(&self) -> String {
        self.find_unique(14)
    }
    fn answer(&self, input: &InputDir, part: &Part) -> Option<&str> {
        match (input.name().as_str(), part) {
            ("Example", Part::One) => Some("7"),
            ("Example", Part::Two) => Some("19"),
            ("Puzzle", Part::One) => Some("1093"),
            ("Puzzle", Part::Two) => Some("3534"),
            _ => unreachable!(),
        }
    }
}

impl Day06 {
    fn find_unique(&self, length: usize) -> String {
        let mut set = HashSet::<char>::new();
        let (i, _) = self
            .data
            .windows(length)
            .find_position(|marker| {
                set.clear();
                marker.iter().all(|&c| set.insert(c))
            })
            .unwrap();
        (i + length).to_string()
    }
}
