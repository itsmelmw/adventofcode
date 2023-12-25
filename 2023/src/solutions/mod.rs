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
use aoc_utils::solutions::Solution;

pub fn get_solution<'i>(day: usize, input: &'i str) -> Box<dyn Solution + 'i> {
    match day {
        1 => Box::new(day01::Day01::parse(input)),
        2 => Box::new(day02::Day02::parse(input)),
        3 => Box::new(day03::Day03::parse(input)),
        4 => Box::new(day04::Day04::parse(input)),
        5 => Box::new(day05::Day05::parse(input)),
        6 => Box::new(day06::Day06::parse(input)),
        7 => Box::new(day07::Day07::parse(input)),
        8 => Box::new(day08::Day08::parse(input)),
        9 => Box::new(day09::Day09::parse(input)),
        10 => Box::new(day10::Day10::parse(input)),
        11 => Box::new(day11::Day11::parse(input)),
        12 => Box::new(day12::Day12::parse(input)),
        13 => Box::new(day13::Day13::parse(input)),
        14 => Box::new(day14::Day14::parse(input)),
        15 => Box::new(day15::Day15::parse(input)),
        16 => Box::new(day16::Day16::parse(input)),
        17 => Box::new(day17::Day17::parse(input)),
        18 => Box::new(day18::Day18::parse(input)),
        19 => Box::new(day19::Day19::parse(input)),
        20 => Box::new(day20::Day20::parse(input)),
        21 => Box::new(day21::Day21::parse(input)),
        22 => Box::new(day22::Day22::parse(input)),
        23 => Box::new(day23::Day23::parse(input)),
        24 => Box::new(day24::Day24::parse(input)),
        25 => Box::new(day25::Day25::parse(input)),
        _ => panic!(),
    }
}
