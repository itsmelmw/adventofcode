// https://adventofcode.com/2023/day/11

use crate::solution::{InputType, Solution};
use itertools::Itertools;

pub struct Day11 {
    xs: Vec<usize>,
    ys: Vec<usize>,
}

impl<'i> Solution<'i> for Day11 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn title(&self) -> &str {
        "Cosmic Expansion"
    }

    fn parse(input: &'i str) -> Self {
        let (mut xs, mut ys) = (Vec::new(), Vec::new());
        input.split('\n').enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                if c == '#' {
                    xs.push(x);
                    ys.push(y);
                }
            })
        });
        // ys is sorted by definition.
        // Could be faster if xs would be guaranteed sorted as well (BinaryHeap?).
        xs.sort();
        Self { xs, ys }
    }

    fn solve_part_1(&self) -> Self::Part1Output {
        self.galaxy_dist_1d(&self.xs, 2) + self.galaxy_dist_1d(&self.ys, 2)
    }
    fn solve_part_2(&self) -> Self::Part2Output {
        self.galaxy_dist_1d(&self.xs, 1_000_000) + self.galaxy_dist_1d(&self.ys, 1_000_000)
    }

    fn solution(
        &self,
        input_type: crate::solution::InputType,
    ) -> (Option<Self::Part1Output>, Option<Self::Part2Output>) {
        match input_type {
            InputType::Examples => (Some(374), Some(82000210)),
            InputType::Puzzles => (Some(9639160), Some(752936133304)),
        }
    }
}

impl Day11 {
    fn galaxy_dist_1d(&self, vec: &[usize], expand_by: usize) -> usize {
        let n = vec.len();
        vec.iter()
            .tuple_windows()
            .enumerate()
            .map(|(i, (v1, v2))| {
                (v2 - v1 + (v2 - v1).saturating_sub(1) * (expand_by - 1)) * (i + 1) * (n - i - 1)
            })
            .sum::<usize>()
    }
}
