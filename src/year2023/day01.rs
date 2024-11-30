// https://adventofcode.com/2023/day/1

use crate::solution::{InputType, Solution};

pub struct Day01 {
    lines: Vec<String>,
}

impl<'i> Solution<'i> for Day01 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn title(&self) -> &str {
        "Trebuchet?!"
    }

    fn parse(input: &'i str) -> Self {
        let lines = input
            .split('\n')
            .map(|line| line.to_string())
            .collect::<Vec<String>>();
        Self { lines }
    }

    fn solve_part_1(&self) -> Self::Part1Output {
        self.lines
            .iter()
            .map(|line| {
                let l = line.find(|c: char| c.is_ascii_digit()).unwrap();
                let r = line.rfind(|c: char| c.is_ascii_digit()).unwrap();
                let first = line.as_bytes().get(l).unwrap() - b'0';
                let last = line.as_bytes().get(r).unwrap() - b'0';
                first as usize * 10 + last as usize
            })
            .sum::<usize>()
    }

    fn solve_part_2(&self) -> Self::Part2Output {
        let words = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        self.lines
            .iter()
            .map(|line| {
                let first = (0..line.len())
                    .find_map(|i| {
                        let slice = &line[i..];
                        if slice.chars().next().unwrap().is_ascii_digit() {
                            return Some((slice.bytes().next().unwrap() - b'0') as usize);
                        }
                        for (v, word) in words.iter().enumerate() {
                            if slice.starts_with(word) {
                                return Some(v + 1);
                            }
                        }
                        None
                    })
                    .unwrap();
                let last = (0..line.len())
                    .find_map(|i| {
                        let slice = &line[..line.len() - i];
                        if slice.chars().last().unwrap().is_ascii_digit() {
                            return Some((slice.bytes().last().unwrap() - b'0') as usize);
                        }
                        for (v, word) in words.iter().enumerate() {
                            if slice.ends_with(word) {
                                return Some(v + 1);
                            }
                        }
                        None
                    })
                    .unwrap();
                first * 10 + last
            })
            .sum::<usize>()
    }

    fn solution(
        &self,
        input_type: crate::solution::InputType,
    ) -> (Option<Self::Part1Output>, Option<Self::Part2Output>) {
        match input_type {
            InputType::Examples => (Some(142), Some(142)),
            InputType::Puzzles => (Some(55108), Some(56324)),
        }
    }
}
