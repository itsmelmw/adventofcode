mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
use crate::{Input, Part};

pub trait Solution {
    fn title(&self) -> &str {
        "Title Unknown"
    }
    fn parse(_input: &str) -> Self
    where
        Self: Sized;

    fn solve(&self, part: &Part) -> String {
        match part {
            Part::One => self.solve_part_1(),
            Part::Two => self.solve_part_2(),
        }
    }
    fn solve_part_1(&self) -> String;
    fn solve_part_2(&self) -> String;

    fn solution(&self, _input: &Input, _part: &Part) -> Option<&str> {
        None
    }
}

pub struct NoSolution;

impl Solution for NoSolution {
    fn parse(_input: &str) -> Self {
        Self
    }
    fn solve_part_1(&self) -> String {
        0.to_string()
    }
    fn solve_part_2(&self) -> String {
        0.to_string()
    }
}

pub fn get_solution(day: usize, input: &str) -> Box<dyn Solution> {
    match day {
        1 => Box::new(day01::Day01::parse(input)),
        2 => Box::new(day02::Day02::parse(input)),
        3 => Box::new(day03::Day03::parse(input)),
        4 => Box::new(day04::Day04::parse(input)),
        5 => Box::new(day05::Day05::parse(input)),
        6 => Box::new(day06::Day06::parse(input)),
        7 => Box::new(day07::Day07::parse(input)),
        8 => Box::new(day08::Day08::parse(input)),
        9 => Box::new(NoSolution::parse(input)),
        10 => Box::new(NoSolution::parse(input)),
        11 => Box::new(NoSolution::parse(input)),
        12 => Box::new(NoSolution::parse(input)),
        13 => Box::new(NoSolution::parse(input)),
        14 => Box::new(NoSolution::parse(input)),
        15 => Box::new(NoSolution::parse(input)),
        16 => Box::new(NoSolution::parse(input)),
        17 => Box::new(NoSolution::parse(input)),
        18 => Box::new(NoSolution::parse(input)),
        19 => Box::new(NoSolution::parse(input)),
        20 => Box::new(NoSolution::parse(input)),
        21 => Box::new(NoSolution::parse(input)),
        22 => Box::new(NoSolution::parse(input)),
        23 => Box::new(NoSolution::parse(input)),
        24 => Box::new(NoSolution::parse(input)),
        25 => Box::new(NoSolution::parse(input)),
        _ => panic!(),
    }
}
