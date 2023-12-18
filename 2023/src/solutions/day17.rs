// https://adventofcode.com/2023/day/17

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use aoc_utils::{
    grids::{Dir, Grid, UPoint},
    solutions::{InputDir, Part, Solution},
};

pub struct State {
    loc: UPoint,
    dir: Dir,
    cost: usize,
    path: Vec<UPoint>,
}

impl Eq for State {}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost.eq(&other.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse cmp since we're using it in a max-heap, but we want a min-heap.
        other.cost.cmp(&self.cost)
    }
}

pub struct Day17 {
    grid: Grid<usize>,
}

impl<'i> Solution<'i> for Day17 {
    fn title(&self) -> &str {
        "Clumsy Crucible"
    }
    fn parse(input: &'i str) -> Self {
        let width = input.find('\n').unwrap();
        let vec = input
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as usize))
            .collect::<Vec<usize>>();
        let grid = Grid::from_vec(vec, width);
        Self { grid }
    }
    fn solve_part_1(&self) -> String {
        self.do_ucs(1, 3).to_string()
    }
    fn solve_part_2(&self) -> String {
        self.do_ucs(4, 10).to_string()
    }
    fn answer(&self, input: &InputDir, part: &Part) -> Option<&str> {
        match (input.name().as_str(), part) {
            ("Example", Part::One) => Some("102"),
            ("Example", Part::Two) => Some("94"),
            ("Puzzle", Part::One) => Some("1039"),
            ("Puzzle", Part::Two) => Some("1201"),
            _ => unreachable!(),
        }
    }
}

impl Day17 {
    fn do_ucs(&self, min_steps: usize, max_steps: usize) -> usize {
        let start_dirs = [Dir::Right, Dir::Down];
        let mut costs = HashMap::new();
        let mut heap = BinaryHeap::from(start_dirs.map(|dir| State {
            loc: UPoint::new(0, 0),
            dir,
            cost: 0,
            path: vec![],
        }));
        while let Some(state) = heap.pop() {
            if state.loc == UPoint::new(self.grid.width() - 1, self.grid.height() - 1) {
                return state.cost;
            }
            if costs
                .get(&(state.loc, state.dir))
                .is_some_and(|&cost| state.cost > cost)
            {
                continue;
            }
            for new_dir in [state.dir.clockwise(), state.dir.counter_clockwise()] {
                let mut new_cost = state.cost;
                for steps in 1..=max_steps {
                    if let Some(new_loc) = self.grid.step_n_in_dir(&state.loc, &new_dir, steps) {
                        new_cost += self.grid.get(&new_loc);
                        if steps < min_steps {
                            continue;
                        }
                        let key = (new_loc, new_dir);
                        if costs.get(&key).map_or(true, |&cost| new_cost < cost) {
                            costs.insert(key, new_cost);
                            let mut path = state.path.clone();
                            path.push(new_loc);
                            heap.push(State {
                                loc: new_loc,
                                dir: new_dir,
                                cost: new_cost,
                                path,
                            });
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        usize::MAX
    }
}
