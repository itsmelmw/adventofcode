// https://adventofcode.com/2022/day/2

use crate::solution::{InputType, Solution};

const OP_ROCK: u8 = b'A';
const OP_PAPER: u8 = b'B';
const OP_SCISSORS: u8 = b'C';

const ME_ROCK: u8 = b'X';
const ME_PAPER: u8 = b'Y';
const ME_SCISSORS: u8 = b'Z';

const GOAL_LOSE: u8 = b'X';
const GOAL_DRAW: u8 = b'Y';
const GOAL_WIN: u8 = b'Z';

const SCORE_ROCK: usize = 1;
const SCORE_PAPER: usize = 2;
const SCORE_SCISSORS: usize = 3;

const SCORE_LOSE: usize = 0;
const SCORE_DRAW: usize = 3;
const SCORE_WIN: usize = 6;

pub struct Day02 {
    data: Vec<(u8, u8)>,
}

impl<'i> Solution<'i> for Day02 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn title(&self) -> &str {
        "Rock Paper Scissors"
    }

    fn parse(input: &'i str) -> Self {
        let data = input
            .split('\n')
            .map(|line| match line.as_bytes() {
                [op, b' ', me] => (*op, *me),
                _ => unreachable!(),
            })
            .collect::<Vec<(u8, u8)>>();
        Self { data }
    }

    fn solve_part_1(&self) -> Self::Part1Output {
        self.data
            .iter()
            .map(|game| match game {
                (OP_ROCK, ME_ROCK) => SCORE_ROCK + SCORE_DRAW,
                (OP_ROCK, ME_PAPER) => SCORE_PAPER + SCORE_WIN,
                (OP_ROCK, ME_SCISSORS) => SCORE_SCISSORS + SCORE_LOSE,
                (OP_PAPER, ME_ROCK) => SCORE_ROCK + SCORE_LOSE,
                (OP_PAPER, ME_PAPER) => SCORE_PAPER + SCORE_DRAW,
                (OP_PAPER, ME_SCISSORS) => SCORE_SCISSORS + SCORE_WIN,
                (OP_SCISSORS, ME_ROCK) => SCORE_ROCK + SCORE_WIN,
                (OP_SCISSORS, ME_PAPER) => SCORE_PAPER + SCORE_LOSE,
                (OP_SCISSORS, ME_SCISSORS) => SCORE_SCISSORS + SCORE_DRAW,
                _ => unreachable!(),
            })
            .sum::<usize>()
    }

    fn solve_part_2(&self) -> Self::Part2Output {
        self.data
            .iter()
            .map(|game| match game {
                (OP_ROCK, GOAL_LOSE) => SCORE_SCISSORS + SCORE_LOSE,
                (OP_ROCK, GOAL_DRAW) => SCORE_ROCK + SCORE_DRAW,
                (OP_ROCK, GOAL_WIN) => SCORE_PAPER + SCORE_WIN,
                (OP_PAPER, GOAL_LOSE) => SCORE_ROCK + SCORE_LOSE,
                (OP_PAPER, GOAL_DRAW) => SCORE_PAPER + SCORE_DRAW,
                (OP_PAPER, GOAL_WIN) => SCORE_SCISSORS + SCORE_WIN,
                (OP_SCISSORS, GOAL_LOSE) => SCORE_PAPER + SCORE_LOSE,
                (OP_SCISSORS, GOAL_DRAW) => SCORE_SCISSORS + SCORE_DRAW,
                (OP_SCISSORS, GOAL_WIN) => SCORE_ROCK + SCORE_WIN,
                _ => unreachable!(),
            })
            .sum::<usize>()
    }

    fn solution(
        &self,
        input_type: crate::solution::InputType,
    ) -> (Option<Self::Part1Output>, Option<Self::Part2Output>) {
        match input_type {
            InputType::Examples => (Some(15), Some(12)),
            InputType::Puzzles => (Some(12645), Some(11756)),
        }
    }
}
