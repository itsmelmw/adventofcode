// https://adventofcode.com/2022/day/18

use crate::solution::{InputType, Day};
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

pub struct Day18 {
    cubes: HashSet<(isize, isize, isize)>,
}

impl<'i> Day<'i> for Day18 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn title(&self) -> &str {
        "Boiling Boulders"
    }

    fn parse(input: &'i str) -> Self {
        let cubes = HashSet::from_iter(input.split('\n').map(|line| {
            line.split(',')
                .map(|num| num.parse::<isize>().unwrap())
                .collect_tuple::<(isize, isize, isize)>()
                .unwrap()
        }));
        Self { cubes }
    }

    fn solve_part_1(&self) -> Self::Part1Output {
        self.cubes
            .iter()
            .map(|loc| {
                self.get_neighbors(*loc)
                    .iter()
                    .map(|loc| !self.cubes.contains(loc) as usize)
                    .sum::<usize>()
            })
            .sum::<usize>()
    }

    fn solve_part_2(&self) -> Self::Part2Output {
        let mut known = HashMap::new();
        self.cubes
            .iter()
            .map(|loc| {
                self.get_neighbors(*loc)
                    .iter()
                    .map(|loc| {
                        (!self.cubes.contains(loc) && !self.is_inside(&mut known, loc)) as usize
                    })
                    .sum::<usize>()
            })
            .sum::<usize>()
    }

    fn solution(
        &self,
        input_type: crate::solution::InputType,
    ) -> (Option<Self::Part1Output>, Option<Self::Part2Output>) {
        match input_type {
            InputType::Examples => (Some(64), Some(58)),
            InputType::Puzzles => (Some(4314), Some(2444)),
        }
    }
}

impl Day18 {
    fn get_neighbors(&self, loc: (isize, isize, isize)) -> [(isize, isize, isize); 6] {
        [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ]
        .map(|(dx, dy, dz)| (loc.0 + dx, loc.1 + dy, loc.2 + dz))
    }

    fn update_known(
        &self,
        known: &mut HashMap<(isize, isize, isize), bool>,
        visited: &HashSet<(isize, isize, isize)>,
        inside: bool,
    ) -> bool {
        visited.iter().for_each(|loc| {
            known.insert(*loc, inside);
        });
        inside
    }

    // We define "inside" as: not being able to reach a border.
    // Border is hardcoded as one of x, y or z being 0 or 21.
    // We calculate this using BFS.
    fn is_inside(
        &self,
        known: &mut HashMap<(isize, isize, isize), bool>,
        loc: &(isize, isize, isize),
    ) -> bool {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back(*loc);
        visited.insert(*loc);

        while let Some(next) = queue.pop_front() {
            if known.contains_key(loc) {
                return known[loc];
            }
            if next.0 <= 0
                || next.0 >= 21
                || next.1 <= 0
                || next.1 >= 21
                || next.2 <= 0
                || next.2 >= 21
            {
                return self.update_known(known, &visited, false);
            }
            for neighbor in self.get_neighbors(next) {
                if !self.cubes.contains(&neighbor) && !visited.contains(&neighbor) {
                    queue.push_back(neighbor);
                    visited.insert(neighbor);
                }
            }
        }
        self.update_known(known, &visited, true)
    }
}
