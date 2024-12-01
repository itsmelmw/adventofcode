// https://adventofcode.com/2022/day/12

use crate::{
    grids::UPoint,
    solution::{Day, InputType},
};
use std::collections::{HashMap, VecDeque};

type StartState = VecDeque<UPoint>;
type Map = Vec<Vec<usize>>;

pub struct Day12 {
    map: Map,
    start1: StartState,
    start2: StartState,
    end: UPoint,
}

impl<'i> Day<'i> for Day12 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn title(&self) -> &str {
        "Hill Climbing Algorithm"
    }

    fn parse(input: &'i str) -> Self {
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

    fn solve_part_1(&self) -> Self::Part1Output {
        self.solve(&mut self.start1.clone())
    }

    fn solve_part_2(&self) -> Self::Part2Output {
        self.solve(&mut self.start2.clone())
    }

    fn solution(
        &self,
        input_type: crate::solution::InputType,
    ) -> (Option<Self::Part1Output>, Option<Self::Part2Output>) {
        match input_type {
            InputType::Examples => (Some(31), Some(29)),
            InputType::Puzzles => (Some(408), Some(339)),
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
