// https://adventofcode.com/2022/day/4

use itertools::Itertools;

use crate::solution::{InputType, Day};

type Assignment = ((usize, usize), (usize, usize));

pub struct Day04 {
    data: Vec<Assignment>,
}

impl<'i> Day<'i> for Day04 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn title(&self) -> &str {
        "Camp Cleanup"
    }

    fn parse(input: &'i str) -> Self {
        let data = input
            .split('\n')
            .map(|line| {
                line.split(',')
                    .map(|range| {
                        range
                            .splitn(2, '-')
                            .map(|num| num.parse::<usize>().unwrap())
                            .collect_tuple()
                            .unwrap()
                    })
                    .collect_tuple()
                    .unwrap()
            })
            .collect::<Vec<Assignment>>();
        Self { data }
    }

    fn solve_part_1(&self) -> Self::Part1Output {
        self.data
            .iter()
            .map(|ranges| self.contains(ranges.0, ranges.1) as usize)
            .sum::<usize>()
    }

    fn solve_part_2(&self) -> Self::Part2Output {
        self.data
            .iter()
            .map(|ranges| self.overlaps(ranges.0, ranges.1) as usize)
            .sum::<usize>()
    }

    fn solution(
        &self,
        input_type: crate::solution::InputType,
    ) -> (Option<Self::Part1Output>, Option<Self::Part2Output>) {
        match input_type {
            InputType::Examples => (Some(2), Some(4)),
            InputType::Puzzles => (Some(462), Some(835)),
        }
    }
}

impl Day04 {
    fn contains(&self, range1: (usize, usize), range2: (usize, usize)) -> bool {
        (range1.0 >= range2.0 && range1.1 <= range2.1)
            || (range2.0 >= range1.0 && range2.1 <= range1.1)
    }

    fn overlaps(&self, range1: (usize, usize), range2: (usize, usize)) -> bool {
        (range1.0 >= range2.0 && range1.0 <= range2.1)
            || (range2.0 >= range1.0 && range2.0 <= range1.1)
    }
}
