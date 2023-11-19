// https://adventofcode.com/2022/day/14

use super::{InputParser, ProblemSolver};
use itertools::Itertools;
use std::collections::HashSet;

fn find_abyss(sand: (usize, usize), points: &mut HashSet<(usize, usize)>, abyss: usize) -> bool {
    if sand.1 == abyss {
        return true;
    }
    for xdiff in [0, -1, 1] {
        let new = ((sand.0 as isize + xdiff) as usize, sand.1 + 1);
        if !points.contains(&new) && find_abyss(new, points, abyss) {
            return true;
        }
    }
    points.insert(sand);
    false
}

fn fill_cave(sand: (usize, usize), points: &mut HashSet<(usize, usize)>, floor: usize) {
    points.insert(sand);
    if sand.1 == floor {
        return;
    }
    for xdiff in [0, -1, 1] {
        let new = ((sand.0 as isize + xdiff) as usize, sand.1 + 1);
        if !points.contains(&new) {
            fill_cave(new, points, floor);
        }
    }
}

fn parse(input: &str) -> (HashSet<(usize, usize)>, usize) {
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

    (points, lowest)
}

fn solve1(points: &mut HashSet<(usize, usize)>, abyss: usize) -> String {
    let orig_size = points.len();
    find_abyss((500, 0), points, abyss);
    (points.len() - orig_size).to_string()
}

fn solve2(points: &mut HashSet<(usize, usize)>, floor: usize) -> String {
    let orig_size = points.len();
    fill_cave((500, 0), points, floor);
    (points.len() - orig_size).to_string()
}

pub struct Parser;

impl InputParser for Parser {
    type S = Solver;
    fn parse(input: &str) -> Solver {
        let (points, lowest) = parse(input);
        Solver { points, lowest }
    }
}

pub struct Solver {
    points: HashSet<(usize, usize)>,
    lowest: usize,
}

impl ProblemSolver for Solver {
    fn solve_part_1(&self) -> String {
        let points = &mut self.points.clone();
        solve1(points, self.lowest)
    }
    fn solve_part_2(&self) -> String {
        let points = &mut self.points.clone();
        solve2(points, self.lowest + 1)
    }
}
