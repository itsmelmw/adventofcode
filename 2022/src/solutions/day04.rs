// https://adventofcode.com/2022/day/4

use itertools::Itertools;

use aoc_utils::solutions::{InputDir, Part, Solution};

type Assignment = ((usize, usize), (usize, usize));

pub struct Day04 {
    data: Vec<Assignment>,
}

impl Solution for Day04 {
    fn title(&self) -> &str {
        "Camp Cleanup"
    }
    fn parse(input: &str) -> Self {
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
    fn solve_part_1(&self) -> String {
        self.data
            .iter()
            .map(|ranges| self.contains(ranges.0, ranges.1) as usize)
            .sum::<usize>()
            .to_string()
    }
    fn solve_part_2(&self) -> String {
        self.data
            .iter()
            .map(|ranges| self.overlaps(ranges.0, ranges.1) as usize)
            .sum::<usize>()
            .to_string()
    }
    fn answer(&self, input: &InputDir, part: &Part) -> Option<&str> {
        match (input.name().as_str(), part) {
            ("Example", Part::One) => Some("2"),
            ("Example", Part::Two) => Some("4"),
            ("Puzzle", Part::One) => Some("462"),
            ("Puzzle", Part::Two) => Some("835"),
            _ => unreachable!(),
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
