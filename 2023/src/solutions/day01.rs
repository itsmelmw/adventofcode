// https://adventofcode.com/2023/day/1

use aoc_utils::solutions::{InputDir, Part, Solution};
// use aoc_utils::{Input, Part};

pub struct Day01 {
    lines: Vec<String>,
}

impl Solution for Day01 {
    fn title(&self) -> &str {
        "Trebuchet?!"
    }
    fn parse(input: &str) -> Self {
        let lines = input
            .split('\n')
            .map(|line| line.to_string())
            .collect::<Vec<String>>();
        Self { lines }
    }
    fn solve_part_1(&self) -> String {
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
            .to_string()
    }
    fn solve_part_2(&self) -> String {
        let words = vec![
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
            .to_string()
    }
    fn solution(&self, input: &InputDir, part: &Part) -> Option<&str> {
        match (input.name().as_str(), part) {
            ("Example", Part::One) => Some("142"),
            ("Example", Part::Two) => Some("142"),
            ("Puzzle", Part::One) => Some("55108"),
            ("Puzzle", Part::Two) => Some("56324"),
            _ => unreachable!(),
        }
    }
}
