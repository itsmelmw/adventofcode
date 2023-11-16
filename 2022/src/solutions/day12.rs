// https://adventofcode.com/2022/day/12

use std::collections::{HashMap, VecDeque};

type Point = (usize, usize);
type StartStates = VecDeque<Point>;
type Map = Vec<Vec<usize>>;

fn parse(input: &str) -> (Map, (StartStates, StartStates), Point) {
    let mut starts = (VecDeque::new(), VecDeque::new());
    let mut end = (0, 0);
    let map = input
        .split('\n')
        .enumerate()
        .map(|(y, line)| {
            line.bytes()
                .enumerate()
                .map(|(x, char)| match char {
                    b'S' => {
                        starts.0.push_back((x, y));
                        starts.1.push_back((x, y));
                        0
                    }
                    b'E' => {
                        end = (x, y);
                        25
                    }
                    b'a' => {
                        starts.1.push_back((x, y));
                        0
                    }
                    byte => (byte - b'a') as usize,
                })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    (map, starts, end)
}

fn get_neighbors(pos: (usize, usize), width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();
    if pos.0 != 0 {
        positions.push((pos.0 - 1, pos.1));
    }
    if pos.0 != width - 1 {
        positions.push((pos.0 + 1, pos.1));
    }
    if pos.1 != 0 {
        positions.push((pos.0, pos.1 - 1));
    }
    if pos.1 != height - 1 {
        positions.push((pos.0, pos.1 + 1));
    }
    positions
}

fn solve12(
    map: &Vec<Vec<usize>>,
    mut queue: VecDeque<(usize, usize)>,
    end: (usize, usize),
) -> String {
    // Initialise BFS
    let height = map.len();
    let width = map[0].len();
    let mut visited = HashMap::<(usize, usize), (usize, usize)>::new();
    for start in queue.iter() {
        visited.insert(*start, *start);
    }

    // Do BFS
    while let Some(pos) = queue.pop_front() {
        if pos == end {
            break;
        }
        let size = map[pos.1][pos.0];

        for new_pos in get_neighbors(pos, width, height) {
            if map[new_pos.1][new_pos.0] <= size + 1 && !visited.contains_key(&new_pos) {
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

pub fn solve(input: &str) -> (String, String) {
    let (map, (start1, start2), end) = parse(input);
    (solve12(&map, start1, end), solve12(&map, start2, end))
}
