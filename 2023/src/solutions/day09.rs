// https://adventofcode.com/2023/day/9

use itertools::Itertools;
use num::Integer;

use aoc_utils::solutions::{InputDir, Part, Solution};

type Diffs = Vec<Vec<isize>>;

pub struct Day09 {
    diffs_vec: Vec<Diffs>,
}

impl Solution for Day09 {
    fn title(&self) -> &str {
        "Mirage Maintenance"
    }
    fn parse(input: &str) -> Self {
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
    fn solve_part_1(&self) -> String {
        self.diffs_vec
            .iter()
            .map(|diffs| diffs.iter().map(|d| d.last().unwrap()).sum::<isize>())
            .sum::<isize>()
            .to_string()
    }
    fn solve_part_2(&self) -> String {
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
            .to_string()
    }
    fn answer(&self, input: &InputDir, part: &Part) -> Option<&str> {
        match (input.name().as_str(), part) {
            ("Example", Part::One) => Some("114"),
            ("Example", Part::Two) => Some("2"),
            ("Puzzle", Part::One) => Some("1584748274"),
            ("Puzzle", Part::Two) => Some("1026"),
            _ => unreachable!(),
        }
    }
}
