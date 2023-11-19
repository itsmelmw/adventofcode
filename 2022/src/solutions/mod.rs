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
use super::Part;

pub trait InputParser {
    type S: ProblemSolver;
    fn parse(input: &str) -> Self::S;
}

pub trait ProblemSolver {
    fn solve(&self, part: &Part) -> String {
        match part {
            Part::One => self.solve_part_1(),
            Part::Two => self.solve_part_2(),
        }
    }
    fn solve_part_1(&self) -> String;
    fn solve_part_2(&self) -> String;
}

pub fn get_solver(day: usize, input: &str) -> Box<dyn ProblemSolver> {
    match day {
        1 => Box::new(day01::Parser::parse(input)),
        2 => Box::new(day02::Parser::parse(input)),
        3 => Box::new(day03::Parser::parse(input)),
        4 => Box::new(day04::Parser::parse(input)),
        5 => Box::new(day05::Parser::parse(input)),
        6 => Box::new(day06::Parser::parse(input)),
        7 => Box::new(day07::Parser::parse(input)),
        8 => Box::new(day08::Parser::parse(input)),
        9 => Box::new(day09::Parser::parse(input)),
        10 => Box::new(day10::Parser::parse(input)),
        11 => Box::new(day11::Parser::parse(input)),
        12 => Box::new(day12::Parser::parse(input)),
        13 => Box::new(day13::Parser::parse(input)),
        14 => Box::new(day14::Parser::parse(input)),
        15 => Box::new(day15::Parser::parse(input)),
        16 => Box::new(day16::Parser::parse(input)),
        17 => Box::new(day17::Parser::parse(input)),
        18 => Box::new(day18::Parser::parse(input)),
        19 => Box::new(day19::Parser::parse(input)),
        20 => Box::new(day20::Parser::parse(input)),
        21 => Box::new(day21::Parser::parse(input)),
        22 => Box::new(day22::Parser::parse(input)),
        23 => Box::new(day23::Parser::parse(input)),
        24 => Box::new(day24::Parser::parse(input)),
        25 => Box::new(day25::Parser::parse(input)),
        _ => panic!(),
    }
}
