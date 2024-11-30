// https://adventofcode.com/2023/day/13

use crate::{
    grids::Grid,
    solution::{InputType, Solution},
};

#[derive(PartialEq, Eq, Debug)]
enum Tile {
    Ash,
    Rock,
}

pub struct Day13 {
    maps: Vec<Grid<Tile>>,
}

impl<'i> Solution<'i> for Day13 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn title(&self) -> &str {
        "Point of Incidence"
    }

    fn parse(input: &'i str) -> Self {
        let maps = input
            .split("\n\n")
            .map(|map| {
                let width = map.find('\n').unwrap();
                let tiles = map
                    .chars()
                    .filter_map(|c| match c {
                        '.' => Some(Tile::Ash),
                        '#' => Some(Tile::Rock),
                        _ => None,
                    })
                    .collect::<Vec<Tile>>();
                Grid::from_vec(tiles, width)
            })
            .collect::<Vec<Grid<Tile>>>();
        Self { maps }
    }

    fn solve_part_1(&self) -> Self::Part1Output {
        self.maps
            .iter()
            .map(|grid| {
                if let Some(mirror_loc) = (1..grid.height()).find(|mirror| {
                    // Check if iters are equal if we place horizontal mirror at row `mirror`.
                    let iter_left = grid
                        .iter_rows()
                        .rev()
                        .skip(grid.height() - mirror)
                        .flatten();
                    let iter_right = grid.iter_rows().skip(*mirror).flatten();
                    iter_left.zip(iter_right).all(|(x, y)| x == y)
                }) {
                    mirror_loc * 100
                } else if let Some(mirror_loc) = (1..grid.width()).find(|mirror| {
                    // Check if iters are equal if we place vertical mirror at column `mirror`.
                    let iter_up = grid.iter_cols().rev().skip(grid.width() - mirror).flatten();
                    let iter_down = grid.iter_cols().skip(*mirror).flatten();
                    iter_up.zip(iter_down).all(|(x, y)| x == y)
                }) {
                    mirror_loc
                } else {
                    panic!()
                }
            })
            .sum::<usize>()
    }

    fn solve_part_2(&self) -> Self::Part2Output {
        self.maps
            .iter()
            .map(|grid| {
                if let Some(mirror_loc) = (1..grid.height()).find(|mirror| {
                    // Check if iters differ by 1 if we place horizontal mirror at row `mirror`.
                    let iter_left = grid
                        .iter_rows()
                        .rev()
                        .skip(grid.height() - mirror)
                        .flatten();
                    let iter_right = grid.iter_rows().skip(*mirror).flatten();
                    iter_left.zip(iter_right).filter(|(x, y)| x != y).count() == 1
                }) {
                    mirror_loc * 100
                } else if let Some(mirror_loc) = (1..grid.width()).find(|mirror| {
                    // Check if iters differ by 1 if we place vertical mirror at column `mirror`.
                    let iter_up = grid.iter_cols().rev().skip(grid.width() - mirror).flatten();
                    let iter_down = grid.iter_cols().skip(*mirror).flatten();
                    iter_up.zip(iter_down).filter(|(x, y)| x != y).count() == 1
                }) {
                    mirror_loc
                } else {
                    panic!()
                }
            })
            .sum::<usize>()
    }

    fn solution(
        &self,
        input_type: crate::solution::InputType,
    ) -> (Option<Self::Part1Output>, Option<Self::Part2Output>) {
        match input_type {
            InputType::Examples => (Some(405), Some(400)),
            InputType::Puzzles => (Some(30158), Some(36474)),
        }
    }
}
