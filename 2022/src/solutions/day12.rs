// https://adventofcode.com/2022/day/12

use aoc_utils::grids::UPoint;
use aoc_utils::solutions::{InputDir, Part, Solution};
use std::collections::{HashMap, VecDeque};

type StartState = VecDeque<UPoint>;
type Map = Vec<Vec<usize>>;

pub struct Day12 {
    map: Map,
    start1: StartState,
    start2: StartState,
    end: UPoint,
}

impl Solution for Day12 {
    fn title(&self) -> &str {
        "Hill Climbing Algorithm"
    }
    fn parse(input: &str) -> Self {
        let mut start1 = VecDeque::new();
        let mut start2 = VecDeque::new();
        let mut end = UPoint::new(0, 0);
        let map = input
            .split('\n')
            .enumerate()
            .map(|(y, line)| {
                line.bytes()
                    .enumerate()
                    .map(|(x, char)| match char {
                        b'S' => {
                            start1.push_back(UPoint::new(x, y));
                            start2.push_back(UPoint::new(x, y));
                            0
                        }
                        b'E' => {
                            end = UPoint::new(x, y);
                            25
                        }
                        b'a' => {
                            start2.push_back(UPoint::new(x, y));
                            0
                        }
                        byte => (byte - b'a') as usize,
                    })
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();
        Self {
            map,
            start1,
            start2,
            end,
        }
    }
    fn solve_part_1(&self) -> String {
        self.solve(&mut self.start1.clone()).to_string()
    }
    fn solve_part_2(&self) -> String {
        self.solve(&mut self.start2.clone()).to_string()
    }
    fn answer(&self, input: &InputDir, part: &Part) -> Option<&str> {
        match (input.name().as_str(), part) {
            ("Example", Part::One) => Some("31"),
            ("Example", Part::Two) => Some("29"),
            ("Puzzle", Part::One) => Some("408"),
            ("Puzzle", Part::Two) => Some("399"),
            _ => unreachable!(),
        }
    }
}

impl Day12 {
    fn solve(&self, queue: &mut StartState) -> usize {
        // Initialise BFS
        let height = self.map.len();
        let width = self.map[0].len();
        let mut visited = HashMap::<UPoint, UPoint>::new();
        for start in queue.iter() {
            visited.insert(*start, *start);
        }

        // Do BFS
        while let Some(pos) = queue.pop_front() {
            if pos == self.end {
                break;
            }
            let size = self.map[pos.y][pos.x];

            for new_pos in pos.neighbors_4_in(width, height) {
                if self.map[new_pos.y][new_pos.x] <= size + 1 && !visited.contains_key(&new_pos) {
                    queue.push_back(new_pos);
                    visited.insert(new_pos, pos);
                }
            }
        }

        // Trace back the shortest path
        let mut path_length = 0;
        let mut curr = self.end;
        let mut prev = visited[&curr];
        while curr != prev {
            curr = prev;
            prev = visited[&curr];
            path_length += 1;
        }
        path_length
    }
}
