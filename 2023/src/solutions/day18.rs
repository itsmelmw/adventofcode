// https://adventofcode.com/2023/day/18

use aoc_utils::{
    grids::{Dir, IPoint},
    solutions::{InputDir, Part, Solution},
};
use itertools::Itertools;

pub struct Day18 {
    path1: Vec<(Dir, isize)>,
    path2: Vec<(Dir, isize)>,
}

impl<'i> Solution<'i> for Day18 {
    fn title(&self) -> &str {
        "Lavaduct Lagoon"
    }
    fn parse(input: &'i str) -> Self {
        let mut path1 = Vec::new();
        let mut path2 = Vec::new();
        input.split('\n').for_each(|line| {
            let (path, hex) = line.split_once(" (#").unwrap();
            let (dir, num) = path.split_once(' ').unwrap();
            let dir = match dir.chars().next().unwrap() {
                'U' => Dir::Up,
                'R' => Dir::Right,
                'D' => Dir::Down,
                'L' => Dir::Left,
                _ => unreachable!(),
            };
            let num = num.parse::<isize>().unwrap();
            path1.push((dir, num));

            let hex = &hex[..hex.len() - 1];
            let (num, dir) = hex.split_at(hex.len() - 1);
            let dir = match dir.chars().next().unwrap() {
                '0' => Dir::Right,
                '1' => Dir::Down,
                '2' => Dir::Left,
                '3' => Dir::Up,
                _ => unreachable!(),
            };
            let num = isize::from_str_radix(num, 16).unwrap();
            path2.push((dir, num));
        });
        Self { path1, path2 }
    }
    fn solve_part_1(&self) -> String {
        self.area(&self.path1).to_string()
    }
    fn solve_part_2(&self) -> String {
        self.area(&self.path2).to_string()
    }
    fn answer(&self, input: &InputDir, part: &Part) -> Option<&str> {
        match (input.name().as_str(), part) {
            ("Example", Part::One) => Some("62"),
            ("Example", Part::Two) => Some("952408144115"),
            ("Puzzle", Part::One) => Some("47045"),
            ("Puzzle", Part::Two) => Some("147839570293376"),
            _ => unreachable!(),
        }
    }
}

impl Day18 {
    fn area(&self, path: &Vec<(Dir, isize)>) -> isize {
        let mut points = Vec::new();
        points.push(IPoint::new(0, 0));
        let mut boundary = 0;
        for (dir, num) in path {
            let point = points.last().unwrap().dir_steps(dir, *num);
            boundary += num;
            points.push(point);
        }

        // Shoelace Formula
        let area = points
            .iter()
            .tuple_windows()
            .map(|(p1, p2)| p1.x * p2.y - p2.x * p1.y)
            .sum::<isize>()
            / 2;

        // Pick's Theorem
        area + (boundary / 2) + 1
    }
}
