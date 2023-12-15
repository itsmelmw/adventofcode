// https://adventofcode.com/2022/day/9

use std::collections::HashSet;

use aoc_utils::solutions::{InputDir, Part, Solution};

pub struct Day09 {
    moves: Vec<(char, usize)>,
}

impl<'i> Solution<'i> for Day09 {
    fn title(&self) -> &str {
        "Rope Bridge"
    }
    fn parse(input: &'i str) -> Self {
        let moves = input
            .split('\n')
            .map(|line| {
                let mut iter = line.chars();
                let dir = iter.next().unwrap();
                let num = iter.skip(1).collect::<String>().parse::<usize>().unwrap();
                (dir, num)
            })
            .collect::<Vec<(char, usize)>>();
        Self { moves }
    }
    fn solve_part_1(&self) -> String {
        self.unique_tail_locs(2).to_string()
    }
    fn solve_part_2(&self) -> String {
        self.unique_tail_locs(10).to_string()
    }
    fn answer(&self, input: &InputDir, part: &Part) -> Option<&str> {
        match (input.name().as_str(), part) {
            ("Example", Part::One) => Some("88"),
            ("Example", Part::Two) => Some("36"),
            ("Puzzle", Part::One) => Some("5960"),
            ("Puzzle", Part::Two) => Some("2327"),
            _ => unreachable!(),
        }
    }
}

impl Day09 {
    fn update_tail(&self, head: (isize, isize), tail: &mut (isize, isize)) -> bool {
        if head.0.abs_diff(tail.0) == 2 || head.1.abs_diff(tail.1) == 2 {
            tail.0 += head.0.cmp(&tail.0) as isize;
            tail.1 += head.1.cmp(&tail.1) as isize;
            return true;
        }
        false
    }

    fn do_move(&self, knots: &mut Vec<(isize, isize)>, dir: char) {
        match dir {
            'L' => knots[0].0 -= 1,
            'R' => knots[0].0 += 1,
            'U' => knots[0].1 -= 1,
            'D' => knots[0].1 += 1,
            _ => unreachable!(),
        }
        for i in 1..knots.len() {
            if !self.update_tail(knots[i - 1], &mut knots[i]) {
                break;
            }
        }
    }

    fn unique_tail_locs(&self, length: usize) -> usize {
        let mut knots = vec![(0, 0); length];
        let mut locs = HashSet::<(isize, isize)>::new();
        for (dir, num) in &self.moves {
            for _ in 0..*num {
                self.do_move(&mut knots, *dir);
                locs.insert(*knots.last().unwrap());
            }
        }
        locs.len()
    }
}
