// https://adventofcode.com/2022/day/22

use super::{InputParser, ProblemSolver};

pub struct Parser;

impl InputParser for Parser {
    type S = Solver;
    fn parse(input: &str) -> Solver {
        Solver {}
    }
}

pub struct Solver {}

impl ProblemSolver for Solver {
    fn solve_part_1(&self) -> String {
        0.to_string()
    }
    fn solve_part_2(&self) -> String {
        0.to_string()
    }
}
