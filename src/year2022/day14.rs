// https://adventofcode.com/2022/day/14

use crate::solution::{InputType, Solution};
use itertools::Itertools;
use std::collections::HashSet;

pub struct Day14 {
    points: HashSet<(usize, usize)>,
    lowest: usize,
}

impl<'i> Solution<'i> for Day14 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn title(&self) -> &str {
        "Regolith Reservoir"
    }

    fn parse(input: &'i str) -> Self {
        let mut points = HashSet::new();
        let mut lowest = 0;
        input.split('\n').for_each(|line| {
            line.split(" -> ")
                .map(|pt| {
                    pt.split(',')
                        .map(|num| num.parse::<usize>().unwrap())
                        .collect_tuple::<(usize, usize)>()
                        .unwrap()
                })
                .tuple_windows::<((usize, usize), (usize, usize))>()
                .for_each(|(pt1, pt2)| {
                    if pt1.1 > lowest {
                        lowest = pt1.1;
                    }
                    let mut pt = pt1;
                    let xdir = ((pt2.0 as isize) - (pt1.0 as isize)).clamp(-1, 1);
                    let ydir = ((pt2.1 as isize) - (pt1.1 as isize)).clamp(-1, 1);
                    while pt != pt2 {
                        points.insert(pt);
                        pt.0 = (pt.0 as isize + xdir) as usize;
                        pt.1 = (pt.1 as isize + ydir) as usize;
                    }
                    points.insert(pt2);
                })
        });
        Self { points, lowest }
    }

    fn solve_part_1(&self) -> Self::Part1Output {
        let points = &mut self.points.clone();
        let orig_size = points.len();
        self.find_abyss((500, 0), points);
        points.len() - orig_size
    }

    fn solve_part_2(&self) -> Self::Part2Output {
        let points = &mut self.points.clone();
        let orig_size = points.len();
        self.fill_cave((500, 0), points);
        points.len() - orig_size
    }

    fn solution(
        &self,
        input_type: crate::solution::InputType,
    ) -> (Option<Self::Part1Output>, Option<Self::Part2Output>) {
        match input_type {
            InputType::Examples => (Some(24), Some(93)),
            InputType::Puzzles => (Some(799), Some(29076)),
        }
    }
}

impl Day14 {
    fn find_abyss(&self, sand: (usize, usize), points: &mut HashSet<(usize, usize)>) -> bool {
        let abyss = self.lowest;
        if sand.1 == abyss {
            return true;
        }
        for xdiff in [0, -1, 1] {
            let new = ((sand.0 as isize + xdiff) as usize, sand.1 + 1);
            if !points.contains(&new) && self.find_abyss(new, points) {
                return true;
            }
        }
        points.insert(sand);
        false
    }

    fn fill_cave(&self, sand: (usize, usize), points: &mut HashSet<(usize, usize)>) {
        let floor = self.lowest + 1;
        points.insert(sand);
        if sand.1 == floor {
            return;
        }
        for xdiff in [0, -1, 1] {
            let new = ((sand.0 as isize + xdiff) as usize, sand.1 + 1);
            if !points.contains(&new) {
                self.fill_cave(new, points);
            }
        }
    }
}
