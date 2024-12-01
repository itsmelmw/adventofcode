// https://adventofcode.com/2022/day/24

use crate::{
    grids::{Dir, IPoint},
    solution::{Day, InputType},
};
use std::collections::{HashSet, VecDeque};

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

pub struct Day24 {
    states: Vec<BlizzardState>,
    width: isize,
    height: isize,
}

impl<'i> Day<'i> for Day24 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn title(&self) -> &str {
        "Blizzard Basin"
    }

    fn parse(input: &'i str) -> Self {
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
        Self {
            states,
            width,
            height,
        }
    }

    fn solve_part_1(&self) -> Self::Part1Output {
        self.do_bfs(
            IPoint::new(0, 0),
            IPoint::new(self.width - 1, self.height - 1),
            0,
        )
    }

    fn solve_part_2(&self) -> Self::Part2Output {
        let iter = self.do_bfs(
            IPoint::new(0, 0),
            IPoint::new(self.width - 1, self.height - 1),
            0,
        );
        let iter = self.do_bfs(
            IPoint::new(self.width - 1, self.height - 1),
            IPoint::new(0, 0),
            iter,
        );
        self.do_bfs(
            IPoint::new(0, 0),
            IPoint::new(self.width - 1, self.height - 1),
            iter,
        )
    }

    fn solution(
        &self,
        input_type: crate::solution::InputType,
    ) -> (Option<Self::Part1Output>, Option<Self::Part2Output>) {
        match input_type {
            InputType::Examples => (Some(18), Some(54)),
            InputType::Puzzles => (Some(305), Some(905)),
        }
    }
}

impl Day24 {
    fn do_bfs(&self, start: IPoint, end: IPoint, start_iter: usize) -> usize {
        let mut i = start_iter;
        loop {
            // As the start and end positions are not part of the map,
            // we loop until we can enter the map, and then start BFS.
            if self.states[i % self.states.len()].contains(&start) {
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
                let blizzards = &self.states[(state.iter + 1) % self.states.len()];
                let mut moves = state.loc.neighbors_4_in(self.width, self.height);
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
}
