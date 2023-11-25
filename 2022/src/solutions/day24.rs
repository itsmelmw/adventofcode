// https://adventofcode.com/2022/day/24

use std::collections::{HashSet, VecDeque};

use crate::solutions::{InputParser, ProblemSolver};
use crate::utils::{Dir, IPoint};

type BlizzardState = HashSet<IPoint>;

struct Blizzard {
    start: IPoint,
    dir: Dir,
}

impl Blizzard {
    fn step(&self, steps: isize, width: isize, height: isize) -> IPoint {
        match self.dir {
            Dir::Up => IPoint::new(self.start.x, (self.start.y - steps).rem_euclid(height)),
            Dir::Right => IPoint::new((self.start.x + steps).rem_euclid(width), self.start.y),
            Dir::Down => IPoint::new(self.start.x, (self.start.y + steps).rem_euclid(height)),
            Dir::Left => IPoint::new((self.start.x - steps).rem_euclid(width), self.start.y),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct State {
    loc: IPoint,
    iter: usize,
}

fn parse(input: &str) -> (Vec<BlizzardState>, isize, isize) {
    let mut width = 0;
    // We skip the start walls, but not the end walls. Thus, start with -1.
    let mut height = -1;
    let blizzards = input
        .split('\n')
        .skip(1)
        .enumerate()
        .flat_map(|(y, l)| {
            height += 1;
            if y == 0 {
                width = l.len() as isize - 2;
            }
            l.chars()
                .skip(1)
                .enumerate()
                .filter_map(move |(x, c)| match c {
                    '^' => Some(Blizzard {
                        start: IPoint::new(x as isize, y as isize),
                        dir: Dir::Up,
                    }),
                    '>' => Some(Blizzard {
                        start: IPoint::new(x as isize, y as isize),
                        dir: Dir::Right,
                    }),
                    'v' => Some(Blizzard {
                        start: IPoint::new(x as isize, y as isize),
                        dir: Dir::Down,
                    }),
                    '<' => Some(Blizzard {
                        start: IPoint::new(x as isize, y as isize),
                        dir: Dir::Left,
                    }),
                    _ => None,
                })
        })
        .collect::<Vec<Blizzard>>();
    let mut states = Vec::new();
    let mut i = 0;
    loop {
        let mut state = BlizzardState::new();
        for blizzard in &blizzards {
            state.insert(blizzard.step(i, width, height));
        }
        states.push(state);
        i += 1;
        if i % width == 0 && i % height == 0 {
            break;
        }
    }
    (states, width, height)
}

fn do_bfs(
    start: IPoint,
    end: IPoint,
    width: isize,
    height: isize,
    start_iter: usize,
    blizzard_states: &Vec<BlizzardState>,
) -> usize {
    let mut i = start_iter;
    loop {
        // As the start and end positions are not part of the map,
        // we loop until we can enter the map, and then start BFS.
        if blizzard_states[i % blizzard_states.len()].contains(&start) {
            i += 1;
            continue;
        }
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let start_state = State {
            loc: start,
            iter: i,
        };
        queue.push_back(start_state.clone());
        visited.insert(start_state);
        while let Some(state) = queue.pop_front() {
            if state.loc == end {
                return state.iter + 1;
            }
            let blizzards = &blizzard_states[(state.iter + 1) % blizzard_states.len()];
            let mut moves = state.loc.neighbors_4_in(width, height);
            moves.push(state.loc);
            for pos in moves {
                if !blizzards.contains(&pos) {
                    let new_state = State {
                        loc: pos,
                        iter: state.iter + 1,
                    };
                    if !visited.contains(&new_state) {
                        visited.insert(new_state.clone());
                        queue.push_back(new_state);
                    }
                }
            }
        }
        i += 1;
    }
}

pub struct Parser;

impl InputParser for Parser {
    type S = Solver;
    fn parse(input: &str) -> Solver {
        let (states, width, height) = parse(input);
        Solver {
            states,
            width,
            height,
        }
    }
}

pub struct Solver {
    states: Vec<BlizzardState>,
    width: isize,
    height: isize,
}

impl ProblemSolver for Solver {
    fn solve_part_1(&self) -> String {
        do_bfs(
            IPoint::new(0, 0),
            IPoint::new(self.width - 1, self.height - 1),
            self.width,
            self.height,
            0,
            &self.states,
        )
        .to_string()
    }
    fn solve_part_2(&self) -> String {
        let iter = do_bfs(
            IPoint::new(0, 0),
            IPoint::new(self.width - 1, self.height - 1),
            self.width,
            self.height,
            0,
            &self.states,
        );
        let iter = do_bfs(
            IPoint::new(self.width - 1, self.height - 1),
            IPoint::new(0, 0),
            self.width,
            self.height,
            iter,
            &self.states,
        );
        let iter = do_bfs(
            IPoint::new(0, 0),
            IPoint::new(self.width - 1, self.height - 1),
            self.width,
            self.height,
            iter,
            &self.states,
        );
        iter.to_string()
    }
}
