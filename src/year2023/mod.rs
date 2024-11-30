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
use crate::solution::{InputType, Solution, Year};

pub struct Year2023;

impl Year for Year2023 {
    fn year(&self) -> usize {
        2023
    }
    fn solve(&self, day: usize, input_type: InputType) {
        let input = self.get_input(day, input_type);
        match day {
            1 => day01::Day01::parse(&input).solve(),
            2 => day02::Day02::parse(&input).solve(),
            3 => day03::Day03::parse(&input).solve(),
            4 => day04::Day04::parse(&input).solve(),
            5 => day05::Day05::parse(&input).solve(),
            6 => day06::Day06::parse(&input).solve(),
            7 => day07::Day07::parse(&input).solve(),
            8 => day08::Day08::parse(&input).solve(),
            9 => day09::Day09::parse(&input).solve(),
            10 => day10::Day10::parse(&input).solve(),
            11 => day11::Day11::parse(&input).solve(),
            12 => day12::Day12::parse(&input).solve(),
            13 => day13::Day13::parse(&input).solve(),
            14 => day14::Day14::parse(&input).solve(),
            15 => day15::Day15::parse(&input).solve(),
            16 => day16::Day16::parse(&input).solve(),
            17 => day17::Day17::parse(&input).solve(),
            18 => day18::Day18::parse(&input).solve(),
            19 => day19::Day19::parse(&input).solve(),
            20 => day20::Day20::parse(&input).solve(),
            21 => day21::Day21::parse(&input).solve(),
            22 => day22::Day22::parse(&input).solve(),
            23 => day23::Day23::parse(&input).solve(),
            24 => day24::Day24::parse(&input).solve(),
            25 => day25::Day25::parse(&input).solve(),
            _ => unreachable!(),
        }
    }
}
