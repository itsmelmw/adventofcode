// https://adventofcode.com/2023/day/21

use std::collections::{HashSet, VecDeque};

use aoc_utils::{
    grids::{Grid, IPoint, UPoint},
    solutions::{InputDir, Part, Solution},
};

#[derive(PartialEq, Eq)]
enum Tile {
    Empty,
    Rock,
}

pub struct Day21 {
    grid: Grid<Tile>,
}

impl<'i> Solution<'i> for Day21 {
    fn title(&self) -> &str {
        "Step Counter"
    }
    fn parse(input: &'i str) -> Self {
        let width = input.find('\n').unwrap();

        let vec = input
            .chars()
            .filter_map(|c| match c {
                '.' | 'S' => Some(Tile::Empty),
                '#' => Some(Tile::Rock),
                _ => None,
            })
            .collect::<Vec<Tile>>();
        let grid = Grid::from_vec(vec, width);
        Self { grid }
    }
    fn solve_part_1(&self) -> String {
        // Hardcode target differently for example input
        let target = if self.grid.width() < 64 { 6 } else { 64 };
        // Start is always in the center for the example and puzzle input.
        let start = UPoint::new(self.grid.width() / 2, self.grid.height() / 2);

        let mut queue = VecDeque::from([(0, start)]);
        let mut visited = HashSet::new();
        let mut finish = HashSet::new();
        while let Some((steps, state)) = queue.pop_front() {
            if steps % 2 == 0 {
                finish.insert(state);
            }
            if steps < target {
                let neighbors = state
                    .neighbors_4()
                    .iter()
                    .filter(|p| *self.grid.get(p) != Tile::Rock && !visited.contains(*p))
                    .copied()
                    .collect::<Vec<UPoint>>();
                for neighbor in neighbors {
                    visited.insert(neighbor);
                    queue.push_back((steps + 1, neighbor));
                }
            }
        }
        finish.len().to_string()
    }
    fn solve_part_2(&self) -> String {
        if self.grid.width() < 64 {
            return "-".to_string();
        }
        // When running this simulation for a smaller number of steps,
        // but with the same remainder of steps for the edge-grids,
        // while tracking the number of end-spots in every grid-clone,
        // you can see an interesting pattern:

        // 5x5 (327 steps)
        //               0  948 5579  941    0
        //             948 6495 7424 6472  941
        //            5588 7424 7388 7424 5559
        //             956 6481 7424 6475  933
        //               0  956 5568  933    0

        // 9x9 (589 steps)
        //     0    0    0  948 5579  941    0    0    0
        //     0    0  948 6495 7424 6472  941    0    0
        //     0  948 6495 7424 7388 7424 6472  941    0
        //   948 6495 7424 7388 7424 7388 7424 6472  941
        //  5588 7424 7388 7424 7388 7424 7388 7424 5559
        //   956 6481 7424 7388 7424 7388 7424 6475  933
        //     0  956 6481 7424 7388 7424 6475  933    0
        //     0    0  956 6481 7424 6475  933    0    0
        //     0    0    0  956 5568  933    0    0    0

        // As you can see, the tip-grids are the same for both,
        // and so are the values for each of the edge-grids (both
        // inner and outer edge grids). Furthermore, the "filled"
        // center grids form a checkerboard-like pattern, alternating
        // between two values. Based on this, we can calculate
        // the sum of these values even for larger grids.

        // This only works because the row and column of the start
        // position are completely empty. For the original example,
        // this code does not work because these are not empty,
        // which is why it's not supported.

        // NOTE: 7x7 has different edge-values, so I omitted it,
        // and decided to only support 5x5, 9x9, 13x13, 17x17 etc,
        // which the input also falls in.

        // Hardcode target differently for example input
        let total_steps = 26501365;
        let mod_target = total_steps % 2;
        let dim = total_steps / self.grid.width();
        if dim % 2 == 1 {
            panic!("Uneven dim not supported");
        }

        let loop_steps = (self.grid.width() / 2) + (self.grid.width() * 2);
        let small_target = loop_steps + ((total_steps - loop_steps) % self.grid.width());

        let iwidth = self.grid.width() as isize;
        let iheight = self.grid.height() as isize;

        let start = IPoint::new(iwidth / 2, iheight / 2);

        let mut queue = VecDeque::from([(0, start)]);
        let mut visited = HashSet::new();
        let mut finish_grid = Grid::from_vec(vec![HashSet::new(); 25], 5);

        while let Some((steps, state)) = queue.pop_front() {
            if steps % 2 == mod_target {
                let grid_loc = UPoint::new(
                    (state.x + iwidth * 2) as usize / self.grid.width(),
                    (state.y + iheight * 2) as usize / self.grid.height(),
                );
                finish_grid.get_mut(&grid_loc).insert(state);
            }
            if steps < small_target {
                let neighbors = state
                    .neighbors_4()
                    .iter()
                    .filter(|p| {
                        *self.grid.get(&UPoint::new(
                            p.x.rem_euclid(iwidth) as usize,
                            p.y.rem_euclid(iheight) as usize,
                        )) != Tile::Rock
                            && !visited.contains(*p)
                    })
                    .copied()
                    .collect::<Vec<IPoint>>();
                for neighbor in neighbors {
                    visited.insert(neighbor);
                    queue.push_back((steps + 1, neighbor));
                }
            }
        }

        let count_vec = finish_grid
            .iter()
            .map(|grid| grid.len())
            .collect::<Vec<usize>>();
        let count_grid = Grid::from_vec(count_vec, 5);
        let tips = [(2, 0), (0, 2), (4, 2), (2, 4)]
            .iter()
            .map(|(x, y)| count_grid.get(&UPoint::new(*x, *y)))
            .sum::<usize>();
        let outer_edges = [(1, 0), (3, 0), (1, 4), (3, 4)]
            .iter()
            .map(|(x, y)| count_grid.get(&UPoint::new(*x, *y)))
            .sum::<usize>();
        let inner_edges = [(1, 1), (3, 1), (1, 3), (3, 3)]
            .iter()
            .map(|(x, y)| count_grid.get(&UPoint::new(*x, *y)))
            .sum::<usize>();
        let outer_center = count_grid.get(&UPoint::new(2, 3));
        let inner_center = count_grid.get(&UPoint::new(2, 2));

        (tips
            + (outer_edges * dim)
            + (inner_edges * (dim - 1))
            + (outer_center * dim * dim)
            + (inner_center * (dim - 1) * (dim - 1)))
            .to_string()
    }
    fn answer(&self, input: &InputDir, part: &Part) -> Option<&str> {
        match (input.name().as_str(), part) {
            ("Example", Part::One) => Some("16"),
            ("Example", Part::Two) => Some("-"),
            ("Puzzle", Part::One) => Some("3646"),
            ("Puzzle", Part::Two) => Some("606188414811259"),
            _ => unreachable!(),
        }
    }
}
