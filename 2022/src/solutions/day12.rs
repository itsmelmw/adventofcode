// https://adventofcode.com/2022/day/12

use crate::solutions::{InputParser, ProblemSolver};
use crate::utils::UPoint;
use std::collections::{HashMap, VecDeque};

type StartState = VecDeque<UPoint>;
type Map = Vec<Vec<usize>>;

fn parse(input: &str) -> (Map, (StartState, StartState), UPoint) {
    let mut starts = (VecDeque::new(), VecDeque::new());
    let mut end = UPoint::new(0, 0);
    let map = input
        .split('\n')
        .enumerate()
        .map(|(y, line)| {
            line.bytes()
                .enumerate()
                .map(|(x, char)| match char {
                    b'S' => {
                        starts.0.push_back(UPoint::new(x, y));
                        starts.1.push_back(UPoint::new(x, y));
                        0
                    }
                    b'E' => {
                        end = UPoint::new(x, y);
                        25
                    }
                    b'a' => {
                        starts.1.push_back(UPoint::new(x, y));
                        0
                    }
                    byte => (byte - b'a') as usize,
                })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    (map, starts, end)
}

fn solve12(map: &Vec<Vec<usize>>, mut queue: VecDeque<UPoint>, end: UPoint) -> String {
    // Initialise BFS
    let height = map.len();
    let width = map[0].len();
    let mut visited = HashMap::<UPoint, UPoint>::new();
    for start in queue.iter() {
        visited.insert(*start, *start);
    }

    // Do BFS
    while let Some(pos) = queue.pop_front() {
        if pos == end {
            break;
        }
        let size = map[pos.y][pos.x];

        for new_pos in pos.neighbors_4_in(width, height) {
            if map[new_pos.y][new_pos.x] <= size + 1 && !visited.contains_key(&new_pos) {
                queue.push_back(new_pos);
                visited.insert(new_pos, pos);
            }
        }
    }

    // Trace back the shortest path
    let mut path_length = 0;
    let mut curr = end;
    let mut prev = visited[&curr];
    while curr != prev {
        curr = prev;
        prev = visited[&curr];
        path_length += 1;
    }
    path_length.to_string()
}

pub struct Parser;

impl InputParser for Parser {
    type S = Solver;
    fn parse(input: &str) -> Solver {
        let (map, (start1, start2), end) = parse(input);
        Solver {
            map,
            start1,
            start2,
            end,
        }
    }
}

pub struct Solver {
    map: Map,
    start1: StartState,
    start2: StartState,
    end: UPoint,
}

impl ProblemSolver for Solver {
    fn solve_part_1(&self) -> String {
        solve12(&self.map, self.start1.clone(), self.end)
    }
    fn solve_part_2(&self) -> String {
        solve12(&self.map, self.start2.clone(), self.end)
    }
}
