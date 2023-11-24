// https://adventofcode.com/2022/day/6

use crate::solutions::{InputParser, ProblemSolver};
use itertools::Itertools;
use std::collections::HashSet;

fn find_unique(data: &[char], length: usize) -> String {
    let mut set = HashSet::<char>::new();
    let (i, _) = data
        .windows(length)
        .find_position(|marker| {
            set.clear();
            marker.iter().all(|&c| set.insert(c))
        })
        .unwrap();
    (i + length).to_string()
}

fn parse(input: &str) -> Vec<char> {
    return input.chars().collect();
}

fn solve1(parsed: &[char]) -> String {
    find_unique(parsed, 4)
}

fn solve2(parsed: &[char]) -> String {
    find_unique(parsed, 14)
}

pub struct Parser;

impl InputParser for Parser {
    type S = Solver;
    fn parse(input: &str) -> Solver {
        let data = parse(input);
        Solver { data }
    }
}

pub struct Solver {
    data: Vec<char>,
}

impl ProblemSolver for Solver {
    fn solve_part_1(&self) -> String {
        solve1(&self.data)
    }
    fn solve_part_2(&self) -> String {
        solve2(&self.data)
    }
}
