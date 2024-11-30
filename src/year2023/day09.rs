// https://adventofcode.com/2023/day/9

use crate::solution::{InputType, Day};
use itertools::Itertools;
use num::Integer;

type Diffs = Vec<Vec<isize>>;

pub struct Day09 {
    diffs_vec: Vec<Diffs>,
}

impl<'i> Day<'i> for Day09 {
    type Part1Output = isize;
    type Part2Output = isize;

    fn title(&self) -> &str {
        "Mirage Maintenance"
    }

    fn parse(input: &'i str) -> Self {
        let diffs_vec = input
            .split('\n')
            .map(|line| {
                line.split(' ')
                    .map(|num| num.parse::<isize>().unwrap())
                    .collect::<Vec<isize>>()
            })
            // Calculate the diffs during parsing as we need them for both parts.
            .map(|seq| {
                let mut diffs = vec![seq];
                while diffs.last().unwrap().iter().any(|&n| n != 0) {
                    diffs.push(
                        diffs
                            .last()
                            .unwrap()
                            .iter()
                            .tuple_windows()
                            .map(|(x, y)| y - x)
                            .collect::<Vec<isize>>(),
                    )
                }
                diffs
            })
            .collect::<Vec<Diffs>>();
        Self { diffs_vec }
    }

    fn solve_part_1(&self) -> Self::Part1Output {
        self.diffs_vec
            .iter()
            .map(|diffs| diffs.iter().map(|d| d.last().unwrap()).sum::<isize>())
            .sum::<isize>()
    }

    fn solve_part_2(&self) -> Self::Part2Output {
        self.diffs_vec
            .iter()
            .map(|diffs| {
                diffs
                    .iter()
                    .map(|d| d.first().unwrap())
                    .enumerate()
                    .fold(
                        0,
                        |total, (i, n)| if i.is_even() { total + n } else { total - n },
                    )
            })
            .sum::<isize>()
    }

    fn solution(
        &self,
        input_type: crate::solution::InputType,
    ) -> (Option<Self::Part1Output>, Option<Self::Part2Output>) {
        match input_type {
            InputType::Examples => (Some(114), Some(2)),
            InputType::Puzzles => (Some(1584748274), Some(1026)),
        }
    }
}
