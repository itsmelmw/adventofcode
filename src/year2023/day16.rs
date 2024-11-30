// https://adventofcode.com/2023/day/16

use std::collections::{HashSet, VecDeque};

use crate::{
    grids::{Dir, Grid, UPoint},
    solution::{InputType, Day},
};

enum Tile {
    Empty,
    Mirror,
    BackMirror,
    HorSplitter,
    VerSplitter,
}

impl Tile {
    fn next_dirs(&self, dir: &Dir) -> Vec<Dir> {
        match self {
            Self::Empty => vec![*dir],
            Self::Mirror => match dir {
                Dir::Up => vec![Dir::Right],
                Dir::Right => vec![Dir::Up],
                Dir::Down => vec![Dir::Left],
                Dir::Left => vec![Dir::Down],
            },
            Self::BackMirror => match dir {
                Dir::Up => vec![Dir::Left],
                Dir::Right => vec![Dir::Down],
                Dir::Down => vec![Dir::Right],
                Dir::Left => vec![Dir::Up],
            },
            Self::HorSplitter => match dir {
                Dir::Up | Dir::Down => vec![Dir::Left, Dir::Right],
                _ => vec![*dir],
            },
            Self::VerSplitter => match dir {
                Dir::Right | Dir::Left => vec![Dir::Up, Dir::Down],
                _ => vec![*dir],
            },
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct State {
    loc: UPoint,
    dir: Dir,
}

pub struct Day16 {
    grid: Grid<Tile>,
}

impl<'i> Day<'i> for Day16 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn title(&self) -> &str {
        "The Floor Will Be Lava"
    }

    fn parse(input: &'i str) -> Self {
        let width = input.find('\n').unwrap();
        let vec = input
            .chars()
            .filter_map(|c| match c {
                '.' => Some(Tile::Empty),
                '/' => Some(Tile::Mirror),
                '\\' => Some(Tile::BackMirror),
                '-' => Some(Tile::HorSplitter),
                '|' => Some(Tile::VerSplitter),
                _ => None,
            })
            .collect::<Vec<Tile>>();
        let grid = Grid::from_vec(vec, width);
        Self { grid }
    }

    fn solve_part_1(&self) -> Self::Part1Output {
        let start = State {
            loc: UPoint::new(0, 0),
            dir: Dir::Right,
        };
        let mut points = HashSet::new();
        let mut mem = HashSet::new();
        let mut queue = VecDeque::from(vec![start]);
        while let Some(state) = queue.pop_front() {
            if mem.contains(&state) {
                continue;
            }
            mem.insert(state.clone());
            points.insert(state.loc);
            let tile = self.grid.get(&state.loc);
            for dir in tile.next_dirs(&state.dir) {
                if (dir == Dir::Up && state.loc.y == 0)
                    || (dir == Dir::Right && state.loc.x == self.grid.width() - 1)
                    || (dir == Dir::Down && state.loc.y == self.grid.height() - 1)
                    || (dir == Dir::Left && state.loc.x == 0)
                {
                    continue;
                }
                let new_state = State {
                    loc: state.loc.dir(&dir),
                    dir,
                };
                queue.push_back(new_state);
            }
        }
        points.len()
    }

    fn solve_part_2(&self) -> Self::Part2Output {
        let mut starts = Vec::new();
        for x in 0..self.grid.width() {
            starts.push(State {
                loc: UPoint::new(x, 0),
                dir: Dir::Down,
            });
            starts.push(State {
                loc: UPoint::new(x, self.grid.height() - 1),
                dir: Dir::Up,
            });
        }
        for y in 0..self.grid.height() {
            starts.push(State {
                loc: UPoint::new(0, y),
                dir: Dir::Right,
            });
            starts.push(State {
                loc: UPoint::new(self.grid.width() - 1, y),
                dir: Dir::Left,
            });
        }
        starts
            .iter()
            .map(|start| {
                let mut points = HashSet::new();
                let mut mem = HashSet::new();
                let mut queue = VecDeque::from(vec![start.clone()]);
                while let Some(state) = queue.pop_front() {
                    if mem.contains(&state) {
                        continue;
                    }
                    mem.insert(state.clone());
                    points.insert(state.loc);
                    let tile = self.grid.get(&state.loc);
                    for dir in tile.next_dirs(&state.dir) {
                        if (dir == Dir::Up && state.loc.y == 0)
                            || (dir == Dir::Right && state.loc.x == self.grid.width() - 1)
                            || (dir == Dir::Down && state.loc.y == self.grid.height() - 1)
                            || (dir == Dir::Left && state.loc.x == 0)
                        {
                            continue;
                        }
                        let new_state = State {
                            loc: state.loc.dir(&dir),
                            dir,
                        };
                        queue.push_back(new_state);
                    }
                }
                points.len()
            })
            .max()
            .unwrap()
    }

    fn solution(
        &self,
        input_type: crate::solution::InputType,
    ) -> (Option<Self::Part1Output>, Option<Self::Part2Output>) {
        match input_type {
            InputType::Examples => (Some(46), Some(51)),
            InputType::Puzzles => (Some(6514), Some(8089)),
        }
    }
}
