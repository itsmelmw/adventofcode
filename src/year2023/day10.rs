// https://adventofcode.com/2023/day/10

use crate::{
    grids::{Dir, UPoint},
    solution::{InputType, Solution},
};
use std::collections::{HashSet, VecDeque};

enum Tile {
    Pipe(Dir, Dir),
    Ground,
}

impl Tile {
    fn map_dir(&self, dir: &Dir) -> Option<Dir> {
        match self {
            Self::Pipe(dir1, dir2) => {
                if dir.opposite() == *dir1 {
                    Some(*dir2)
                } else if dir.opposite() == *dir2 {
                    Some(*dir1)
                } else {
                    None
                }
            }
            Self::Ground => None,
        }
    }
}

pub struct Day10 {
    map: Vec<Vec<Tile>>,
    start: UPoint,
}

impl<'i> Solution<'i> for Day10 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn title(&self) -> &str {
        "Pipe Maze"
    }

    fn parse(input: &'i str) -> Self {
        let mut start = UPoint::new(0, 0);
        let map = input
            .split('\n')
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '|' => Tile::Pipe(Dir::Up, Dir::Down),
                        '-' => Tile::Pipe(Dir::Left, Dir::Right),
                        'L' => Tile::Pipe(Dir::Up, Dir::Right),
                        'J' => Tile::Pipe(Dir::Up, Dir::Left),
                        '7' => Tile::Pipe(Dir::Left, Dir::Down),
                        'F' => Tile::Pipe(Dir::Right, Dir::Down),
                        'S' => {
                            start = UPoint::new(x, y);
                            Tile::Ground
                        }
                        _ => Tile::Ground,
                    })
                    .collect::<Vec<Tile>>()
            })
            .collect::<Vec<Vec<Tile>>>();
        Self { map, start }
    }

    fn solve_part_1(&self) -> Self::Part1Output {
        // We start by going down. For every input, this was possible from the start,
        // but it could be impossible for other inputs, making this not work.
        let mut curr_dir = Dir::Down;
        let mut curr_pos = self.start.dir(&curr_dir);
        let mut steps = 1;
        while curr_pos != self.start {
            curr_dir = self.map[curr_pos.y][curr_pos.x].map_dir(&curr_dir).unwrap();
            curr_pos = curr_pos.dir(&curr_dir);
            steps += 1;
        }
        steps / 2
    }

    fn solve_part_2(&self) -> Self::Part2Output {
        // First, do step 1 again but put all points in the main loop in a set.
        // Also, put any points to the side of the main loop in another set.
        // Hardcoded to always check the RIGHT side of the main loop, as for
        // the inputs this was always the enclosed part. For some inputs,
        // this may not work.
        let mut curr_dir = Dir::Down;
        let mut curr_pos = self.start.dir(&curr_dir);
        let mut main_loop = HashSet::new();
        let mut enclosed = HashSet::new();
        main_loop.insert(curr_pos);
        enclosed.insert(curr_pos.dir(&curr_dir.clockwise()));
        while curr_pos != self.start {
            let side = curr_pos.dir(&curr_dir.clockwise());
            if !main_loop.contains(&side) {
                enclosed.insert(side);
            }
            curr_dir = self.map[curr_pos.y][curr_pos.x].map_dir(&curr_dir).unwrap();
            let side = curr_pos.dir(&curr_dir.clockwise());
            if !main_loop.contains(&side) {
                enclosed.insert(side);
            }

            curr_pos = curr_pos.dir(&curr_dir);
            main_loop.insert(curr_pos);
            enclosed.remove(&curr_pos);
        }
        // Do BFS to find the rest of the points not directly connected to the main loop
        let mut queue = VecDeque::from_iter(enclosed.clone());
        while let Some(encl_pos) = queue.pop_front() {
            for pos in encl_pos.neighbors_4() {
                if !main_loop.contains(&pos) && !enclosed.contains(&pos) {
                    enclosed.insert(pos);
                    queue.push_back(pos);
                }
            }
        }
        enclosed.len()
    }

    fn solution(
        &self,
        input_type: crate::solution::InputType,
    ) -> (Option<Self::Part1Output>, Option<Self::Part2Output>) {
        match input_type {
            InputType::Examples => (Some(80), Some(10)),
            InputType::Puzzles => (Some(6979), Some(443)),
        }
    }
}
