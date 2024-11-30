// https://adventofcode.com/2022/day/6

use crate::solution::{InputType, Day};
use itertools::Itertools;
use std::collections::HashSet;

pub struct Day06 {
    data: Vec<char>,
}

impl<'i> Day<'i> for Day06 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn title(&self) -> &str {
        "Tuning Trouble"
    }

    fn parse(input: &'i str) -> Self {
        let data = input.chars().collect();
        Self { data }
    }

    fn solve_part_1(&self) -> Self::Part1Output {
        self.find_unique(4)
    }

    fn solve_part_2(&self) -> Self::Part2Output {
        self.find_unique(14)
    }

    fn solution(
        &self,
        input_type: crate::solution::InputType,
    ) -> (Option<Self::Part1Output>, Option<Self::Part2Output>) {
        match input_type {
            InputType::Examples => (Some(7), Some(19)),
            InputType::Puzzles => (Some(1093), Some(3534)),
        }
    }
}

impl Day06 {
    fn find_unique(&self, length: usize) -> usize {
        let mut set = HashSet::<char>::new();
        let (i, _) = self
            .data
            .windows(length)
            .find_position(|marker| {
                set.clear();
                marker.iter().all(|&c| set.insert(c))
            })
            .unwrap();
        i + length
    }
}
