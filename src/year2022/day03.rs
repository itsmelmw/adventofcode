// https://adventofcode.com/2022/day/3

use crate::solution::{InputType, Day};
use std::collections::HashSet;

pub struct Day03 {
    data: Vec<Vec<usize>>,
}

impl<'i> Day<'i> for Day03 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn title(&self) -> &str {
        "Rucksack Reorganization"
    }

    fn parse(input: &'i str) -> Self {
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

    fn solve_part_1(&self) -> Self::Part1Output {
        self.data
            .iter()
            .map(|sack| {
                let half = sack.len() / 2;
                let first = HashSet::<usize>::from_iter(sack[..half].iter().copied());
                let second = HashSet::<usize>::from_iter(sack[half..].iter().copied());
                *first.intersection(&second).next().unwrap()
            })
            .sum::<usize>()
    }

    fn solve_part_2(&self) -> Self::Part2Output {
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
    }

    fn solution(
        &self,
        input_type: crate::solution::InputType,
    ) -> (Option<Self::Part1Output>, Option<Self::Part2Output>) {
        match input_type {
            InputType::Examples => (Some(157), Some(70)),
            InputType::Puzzles => (Some(7742), Some(2276)),
        }
    }
}
