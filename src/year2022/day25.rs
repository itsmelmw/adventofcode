// https://adventofcode.com/2022/day/25

use crate::solution::{InputType, Day};

struct Snafu(String);

impl From<&Snafu> for isize {
    fn from(value: &Snafu) -> Self {
        value
            .0
            .chars()
            .rev()
            .enumerate()
            .map(|(i, c)| match c {
                '=' => -2,
                '-' => -1,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                _ => unreachable!(),
            } * 5isize.pow(i as u32))
            .sum()
    }
}

impl From<isize> for Snafu {
    fn from(value: isize) -> Self {
        let mut val = value;
        let mut vec = Vec::new();
        while val > 0 {
            vec.push(val % 5);
            val /= 5;
        }
        let mut c = 0;
        let mut result = vec
            .iter()
            .map(|d| match d + c {
                0 => {
                    c = 0;
                    '0'
                }
                1 => {
                    c = 0;
                    '1'
                }
                2 => {
                    c = 0;
                    '2'
                }
                3 => {
                    c = 1;
                    '='
                }
                4 => {
                    c = 1;
                    '-'
                }
                5 => {
                    c = 1;
                    '0'
                }
                _ => unreachable!(),
            })
            .collect::<String>();
        if c == 1 {
            result.push('1');
        }
        Snafu(result.chars().rev().collect::<String>())
    }
}

pub struct Day25 {
    nums: Vec<Snafu>,
}

impl<'i> Day<'i> for Day25 {
    type Part1Output = String;
    type Part2Output = usize;

    fn title(&self) -> &str {
        "Full of Hot Air"
    }

    fn parse(input: &'i str) -> Self {
        let nums = input
            .split('\n')
            .map(|l| Snafu(l.to_string()))
            .collect::<Vec<Snafu>>();
        Self { nums }
    }

    fn solve_part_1(&self) -> Self::Part1Output {
        Snafu::from(self.nums.iter().map(isize::from).sum::<isize>()).0
    }

    fn solve_part_2(&self) -> Self::Part2Output {
        0
    }

    fn solution(
        &self,
        input_type: crate::solution::InputType,
    ) -> (Option<Self::Part1Output>, Option<Self::Part2Output>) {
        match input_type {
            InputType::Examples => (Some("2=-1=0".into()), Some(0)),
            InputType::Puzzles => (Some("2=12-100--1012-0=012".into()), Some(0)),
        }
    }
    // fn answer(&self, input: &InputDir, part: &Part) -> Option<&str> {
    //     match (input.name().as_str(), part) {
    //         ("Example", Part::One) => Some("2=-1=0"),
    //         ("Example", Part::Two) => Some("-"),
    //         ("Puzzle", Part::One) => Some("2=12-100--1012-0=012"),
    //         ("Puzzle", Part::Two) => Some("-"),
    //         _ => unreachable!(),
    //     }
    // }
}
