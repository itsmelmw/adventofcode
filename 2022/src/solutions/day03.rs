// https://adventofcode.com/2022/day/3

use super::{InputParser, ProblemSolver};
use std::collections::HashSet;

fn parse(input: &str) -> Vec<Vec<usize>> {
    return input
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
}

fn solve1(parsed: &[Vec<usize>]) -> String {
    return parsed
        .iter()
        .map(|sack| {
            let half = sack.len() / 2;
            let first = HashSet::<usize>::from_iter(sack[..half].iter().copied());
            let second = HashSet::<usize>::from_iter(sack[half..].iter().copied());
            *first.intersection(&second).next().unwrap()
        })
        .sum::<usize>()
        .to_string();
}

fn solve2(parsed: &[Vec<usize>]) -> String {
    return parsed
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
        .to_string();
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
    data: Vec<Vec<usize>>,
}

impl ProblemSolver for Solver {
    fn solve_part_1(&self) -> String {
        solve1(&self.data)
    }
    fn solve_part_2(&self) -> String {
        solve2(&self.data)
    }
}
