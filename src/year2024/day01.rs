// https://adventofcode.com/2024/day/1

use crate::solution::{Day, InputType};
use std::collections::HashMap;

pub struct Day01 {
    left: Vec<usize>,
    right: Vec<usize>,
    right_count: HashMap<usize, usize>,
}

impl<'i> Day<'i> for Day01 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn title(&self) -> &str {
        "Historian Hysteria"
    }

    fn parse(input: &'i str) -> Self {
        let mut right_count = HashMap::new();
        let (left, right): (Vec<usize>, Vec<usize>) = input
            .split('\n')
            .map(|line| {
                line.split_once("   ")
                    .map(|(l, r)| {
                        let l = l.parse::<usize>().unwrap();
                        let r = r.parse::<usize>().unwrap();
                        *right_count.entry(r).or_insert(0) += 1;
                        (l, r)
                    })
                    .unwrap()
            })
            .unzip();
        Self {
            left,
            right,
            right_count,
        }
    }

    fn solve_part_1(&self) -> Self::Part1Output {
        self.left
            .iter()
            .zip(self.right.iter())
            .map(|(l, r)| l.abs_diff(*r))
            .sum()
    }

    fn solve_part_2(&self) -> Self::Part2Output {
        self.left
            .iter()
            .map(|l| l * self.right_count.get(l).unwrap_or(&0))
            .sum()
    }

    fn solution(
        &self,
        input_type: crate::solution::InputType,
    ) -> (Option<Self::Part1Output>, Option<Self::Part2Output>) {
        match input_type {
            InputType::Examples => (Some(13), Some(31)),
            InputType::Puzzles => (Some(31567768), Some(24931009)),
        }
    }
}
