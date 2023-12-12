// https://adventofcode.com/2022/day/3

use aoc_utils::solutions::{InputDir, Part, Solution};
use std::collections::HashSet;

pub struct Day03 {
    data: Vec<Vec<usize>>,
}

impl Solution for Day03 {
    fn title(&self) -> &str {
        "Rucksack Reorganization"
    }
    fn parse(input: &str) -> Self {
        let data = input
            .split('\n')
            .map(|line| {
                line.as_bytes()
                    .iter()
                    .map(|b| match b {
                        b'a'..=b'z' => (b - b'a' + 1) as usize,
                        b'A'..=b'Z' => (b - b'A' + 27) as usize,
                        _ => unreachable!(),
                    })
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();
        Self { data }
    }
    fn solve_part_1(&self) -> String {
        self.data
            .iter()
            .map(|sack| {
                let half = sack.len() / 2;
                let first = HashSet::<usize>::from_iter(sack[..half].iter().copied());
                let second = HashSet::<usize>::from_iter(sack[half..].iter().copied());
                *first.intersection(&second).next().unwrap()
            })
            .sum::<usize>()
            .to_string()
    }
    fn solve_part_2(&self) -> String {
        self.data
            .chunks(3)
            .map(|group| {
                *group
                    .iter()
                    .map(|x| HashSet::<&usize>::from_iter(x.iter()))
                    .reduce(|acc, val| acc.intersection(&val).copied().collect())
                    .unwrap()
                    .iter()
                    .next()
                    .unwrap()
            })
            .sum::<usize>()
            .to_string()
    }
    fn answer(&self, input: &InputDir, part: &Part) -> Option<&str> {
        match (input.name().as_str(), part) {
            ("Example", Part::One) => Some("157"),
            ("Example", Part::Two) => Some("70"),
            ("Puzzle", Part::One) => Some("7742"),
            ("Puzzle", Part::Two) => Some("2276"),
            _ => unreachable!(),
        }
    }
}
