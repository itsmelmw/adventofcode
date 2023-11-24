// https://adventofcode.com/2022/day/5

use crate::solutions::{InputParser, ProblemSolver};
use itertools::Itertools;

type Stacks = Vec<Vec<char>>;
type Moves = Vec<(usize, usize, usize)>;

fn parse(input: &str) -> (Stacks, Moves) {
    let (stack_lines, move_lines) = input.split("\n\n").collect_tuple().unwrap();
    let mut stack_iter = stack_lines.rsplit('\n');
    let mut stacks = Vec::new();
    let stack_amt = (stack_iter.next().unwrap().len() + 1) / 4;

    for _ in 0..stack_amt {
        stacks.push(Vec::new());
    }
    for line in stack_iter {
        for (i, char) in line.chars().skip(1).step_by(4).enumerate() {
            match char {
                ' ' => (),
                _ => stacks[i].push(char),
            }
        }
    }
    let moves = move_lines
        .split('\n')
        .map(|mov_line| {
            let mov = mov_line.split(' ').collect::<Vec<&str>>();
            (
                mov[1].parse::<usize>().unwrap(),
                mov[3].parse::<usize>().unwrap(),
                mov[5].parse::<usize>().unwrap(),
            )
        })
        .collect::<Moves>();

    (stacks, moves)
}

fn solve1(stacks: &mut Stacks, moves: &Moves) -> String {
    for mov in moves {
        for _ in 0..mov.0 {
            let char = stacks[mov.1 - 1].pop().unwrap();
            stacks[mov.2 - 1].push(char);
        }
    }
    let mut out = String::new();
    for stack in stacks {
        out.push(stack.pop().unwrap());
    }
    out
}

fn solve2(stacks: &mut Stacks, moves: &Moves) -> String {
    for mov in moves {
        let stack = &mut stacks[mov.1 - 1];
        let mut chars = stack.split_off(stack.len() - mov.0);
        stacks[mov.2 - 1].append(&mut chars);
    }
    let mut out = String::new();
    for stack in stacks {
        out.push(stack.pop().unwrap());
    }
    out
}

pub struct Parser;

impl InputParser for Parser {
    type S = Solver;
    fn parse(input: &str) -> Solver {
        let (stacks, moves) = parse(input);
        Solver { stacks, moves }
    }
}

pub struct Solver {
    stacks: Stacks,
    moves: Moves,
}

impl ProblemSolver for Solver {
    fn solve_part_1(&self) -> String {
        let stacks = &mut self.stacks.clone();
        solve1(stacks, &self.moves)
    }
    fn solve_part_2(&self) -> String {
        let stacks = &mut self.stacks.clone();
        solve2(stacks, &self.moves)
    }
}
