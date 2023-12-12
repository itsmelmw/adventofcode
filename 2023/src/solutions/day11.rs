// https://adventofcode.com/2023/day/11

use aoc_utils::solutions::{InputDir, Part, Solution};
use itertools::Itertools;

pub struct Day11 {
    xs: Vec<usize>,
    ys: Vec<usize>,
}

impl Solution for Day11 {
    fn title(&self) -> &str {
        "Cosmic Expansion"
    }
    fn parse(input: &str) -> Self {
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
    fn solve_part_1(&self) -> String {
        (self.galaxy_dist_1d(&self.xs, 2) + self.galaxy_dist_1d(&self.ys, 2)).to_string()
    }
    fn solve_part_2(&self) -> String {
        (self.galaxy_dist_1d(&self.xs, 1_000_000) + self.galaxy_dist_1d(&self.ys, 1_000_000))
            .to_string()
    }
    fn answer(&self, input: &InputDir, part: &Part) -> Option<&str> {
        match (input.name().as_str(), part) {
            ("Example", Part::One) => Some("374"),
            ("Example", Part::Two) => Some("82000210"),
            ("Puzzle", Part::One) => Some("9639160"),
            ("Puzzle", Part::Two) => Some("752936133304"),
            _ => unreachable!(),
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
