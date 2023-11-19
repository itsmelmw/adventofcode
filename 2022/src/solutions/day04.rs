// https://adventofcode.com/2022/day/4

use itertools::Itertools;

use super::{InputParser, ProblemSolver};

type Assignment = ((usize, usize), (usize, usize));

fn contains(range1: (usize, usize), range2: (usize, usize)) -> bool {
    (range1.0 >= range2.0 && range1.1 <= range2.1) || (range2.0 >= range1.0 && range2.1 <= range1.1)
}

fn overlaps(range1: (usize, usize), range2: (usize, usize)) -> bool {
    (range1.0 >= range2.0 && range1.0 <= range2.1) || (range2.0 >= range1.0 && range2.0 <= range1.1)
}

fn parse(input: &str) -> Vec<Assignment> {
    return input
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
}

fn solve1(parsed: &[Assignment]) -> String {
    return parsed
        .iter()
        .map(|ranges| contains(ranges.0, ranges.1) as usize)
        .sum::<usize>()
        .to_string();
}

fn solve2(parsed: &[Assignment]) -> String {
    return parsed
        .iter()
        .map(|ranges| overlaps(ranges.0, ranges.1) as usize)
        .sum::<usize>()
        .to_string();
}

pub struct Parser;

impl InputParser for Parser {
    type S = Solver;
    fn parse(input: &str) -> Solver {
        let data = parse(input);
        Solver { data }
    }
}

pub struct Solver {
    data: Vec<Assignment>,
}

impl ProblemSolver for Solver {
    fn solve_part_1(&self) -> String {
        solve1(&self.data)
    }
    fn solve_part_2(&self) -> String {
        solve2(&self.data)
    }
}
