// https://adventofcode.com/2023/day/23

use crate::{
    grids::{Dir, Grid, UPoint},
    solution::{InputType, Day},
};
use std::collections::{HashMap, HashSet, VecDeque};

enum Tile {
    Empty,
    Wall,
    Slope(Dir),
}

pub struct Day23 {
    graph: HashMap<UPoint, Vec<(UPoint, usize, bool)>>,
    start: UPoint,
    end: UPoint,
}

impl<'i> Day<'i> for Day23 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn title(&self) -> &str {
        "A Long Walk"
    }

    fn parse(input: &'i str) -> Self {
        let width = input.find('\n').unwrap();
        let vec = input
            .chars()
            .filter_map(|c| match c {
                '.' => Some(Tile::Empty),
                '#' => Some(Tile::Wall),
                '>' => Some(Tile::Slope(Dir::Right)),
                'v' => Some(Tile::Slope(Dir::Down)),
                '<' => Some(Tile::Slope(Dir::Left)),
                '^' => Some(Tile::Slope(Dir::Up)),
                _ => None,
            })
            .collect::<Vec<Tile>>();
        let grid = Grid::from_vec(vec, width);
        let start = UPoint::new(1, 0);
        let end = UPoint::new(grid.width() - 2, grid.height() - 1);
        let graph = Day23::to_graph(grid, UPoint::new(1, 0));
        Self { graph, start, end }
    }

    fn solve_part_1(&self) -> Self::Part1Output {
        self.longest_path(self.start, &HashSet::new(), false)
            .unwrap()
    }

    fn solve_part_2(&self) -> Self::Part2Output {
        self.longest_path(self.start, &HashSet::new(), true)
            .unwrap()
    }

    fn solution(
        &self,
        input_type: crate::solution::InputType,
    ) -> (Option<Self::Part1Output>, Option<Self::Part2Output>) {
        match input_type {
            InputType::Examples => (Some(94), Some(154)),
            InputType::Puzzles => (Some(2366), Some(6682)),
        }
    }
}

impl Day23 {
    fn to_graph(grid: Grid<Tile>, start: UPoint) -> HashMap<UPoint, Vec<(UPoint, usize, bool)>> {
        let mut queue = VecDeque::from([start]);
        let mut map = HashMap::new();
        while let Some(conjunction) = queue.pop_front() {
            let mut neighbors = Vec::new();
            for dir in Dir::iter() {
                if let Some(loc) = grid.step_in_dir(&conjunction, dir) {
                    if matches!(grid.get(&loc), Tile::Wall) {
                        continue;
                    }
                } else {
                    continue;
                }
                let mut next_dirs = vec![*dir];
                let mut cur_loc = conjunction;
                let mut uses_slope = false;
                let mut steps = 0;
                while next_dirs.len() == 1 {
                    let cur_dir = next_dirs[0];
                    if let Some(loc) = grid.step_in_dir(&cur_loc, &cur_dir) {
                        if let Tile::Slope(dir) = grid.get(&loc) {
                            if *dir != cur_dir {
                                uses_slope = true;
                            }
                        }
                        steps += 1;
                        cur_loc = loc;
                        next_dirs = Dir::iter()
                            .copied()
                            .filter(|dir| {
                                *dir != cur_dir.opposite()
                                    && if let Some(loc) = grid.step_in_dir(&cur_loc, dir) {
                                        !matches!(grid.get(&loc), Tile::Wall)
                                    } else {
                                        false
                                    }
                            })
                            .collect::<Vec<Dir>>();
                    } else {
                        break;
                    }
                }
                if steps > 0 {
                    neighbors.push((cur_loc, steps, uses_slope));
                    if !map.contains_key(&cur_loc) {
                        queue.push_back(cur_loc);
                    }
                }
            }
            map.insert(conjunction, neighbors);
        }
        map
    }
    fn longest_path(
        &self,
        loc: UPoint,
        visited: &HashSet<UPoint>,
        use_slopes: bool,
    ) -> Option<usize> {
        if loc == self.end {
            return Some(0);
        }
        self.graph
            .get(&loc)
            .unwrap()
            .iter()
            .filter_map(|(new_loc, cost, uses_slope)| {
                if (!use_slopes && *uses_slope) || visited.contains(new_loc) {
                    return None;
                }
                let mut visited = visited.clone();
                visited.insert(*new_loc);
                self.longest_path(*new_loc, &visited, use_slopes)
                    .map(|v| v + cost)
            })
            .max()
    }
}
