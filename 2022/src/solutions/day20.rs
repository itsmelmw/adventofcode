// https://adventofcode.com/2022/day/20

use super::{InputParser, ProblemSolver};

fn parse(input: &str) -> Vec<isize> {
    return input
        .split('\n')
        .map(|line| line.parse().unwrap())
        .collect();
}

fn mix_and_sum(list: &Vec<isize>, iters: usize) -> isize {
    let mut mixed = list.iter().enumerate().collect::<Vec<(usize, &isize)>>();
    for iter in 0..iters {
        for idx in 0..list.len() {
            let num = list[idx];
            let cur_idx = mixed.iter().position(|(i, _)| *i == idx).unwrap();
            let new_idx = (cur_idx as isize + num).rem_euclid((list.len() - 1) as isize) as usize;

            let val = mixed.remove(cur_idx);
            mixed.insert(new_idx, val);
        }
    }

    let zero_idx = mixed.iter().position(|(_, &v)| v == 0).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|i| {
            let idx = (zero_idx as isize + i).rem_euclid(list.len() as isize) as usize;
            *mixed[idx].1
        })
        .sum::<isize>()
}

fn solve1(parsed: &Vec<isize>) -> String {
    mix_and_sum(parsed, 1).to_string()
}

fn solve2(parsed: &[isize]) -> String {
    let decrypted = parsed.iter().map(|i| i * 811589153).collect::<Vec<isize>>();
    mix_and_sum(&decrypted, 10).to_string()
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
    data: Vec<isize>,
}

impl ProblemSolver for Solver {
    fn solve_part_1(&self) -> String {
        solve1(&self.data)
    }
    fn solve_part_2(&self) -> String {
        solve2(&self.data)
    }
}
