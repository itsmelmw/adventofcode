// https://adventofcode.com/2023/day/13

use aoc_utils::grids::Grid;
use aoc_utils::solutions::{InputDir, Part, Solution};

#[derive(PartialEq, Eq, Debug)]
enum Tile {
    Ash,
    Rock,
}

pub struct Day13 {
    maps: Vec<Grid<Tile>>,
}

impl Solution for Day13 {
    fn title(&self) -> &str {
        "Point of Incidence"
    }
    fn parse(input: &str) -> Self {
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
    fn solve_part_1(&self) -> String {
        self.maps
            .iter()
            .map(|grid| {
                if let Some(l) = (1..grid.height()).find(|r| {
                    // Search for horizontal mirrors
                    grid.iter_rows()
                        .skip(*r)
                        .zip(grid.iter_rows().rev().skip(grid.height() - r))
                        .all(|(i1, i2)| i1.eq(i2))
                }) {
                    l * 100
                } else if let Some(l) = (1..grid.width()).find(|r| {
                    // Search for vertical mirrors
                    grid.iter_cols()
                        .skip(*r)
                        .zip(grid.iter_cols().rev().skip(grid.width() - r))
                        .all(|(i1, i2)| i1.eq(i2))
                }) {
                    l
                } else {
                    panic!()
                }
            })
            .sum::<usize>()
            .to_string()
    }
    fn solve_part_2(&self) -> String {
        self.maps
            .iter()
            .map(|grid| {
                if let Some(l) = (1..grid.height()).find(|r| {
                    // Search for horizontal mirrors
                    grid.iter_rows()
                        .skip(*r)
                        .zip(grid.iter_rows().rev().skip(grid.height() - r))
                        .map(|(i1, i2)| {
                            i1.zip(i2)
                                .map(|(t1, t2)| (t1 != t2) as usize)
                                .sum::<usize>()
                        })
                        .sum::<usize>()
                        == 1
                }) {
                    l * 100
                } else if let Some(l) = (1..grid.width()).find(|r| {
                    // Search for vertical mirrors
                    grid.iter_cols()
                        .skip(*r)
                        .zip(grid.iter_cols().rev().skip(grid.width() - r))
                        .map(|(i1, i2)| {
                            i1.zip(i2)
                                .map(|(t1, t2)| (t1 != t2) as usize)
                                .sum::<usize>()
                        })
                        .sum::<usize>()
                        == 1
                }) {
                    l
                } else {
                    panic!()
                }
            })
            .sum::<usize>()
            .to_string()
    }
    fn answer(&self, input: &InputDir, part: &Part) -> Option<&str> {
        match (input.name().as_str(), part) {
            ("Example", Part::One) => Some("405"),
            ("Example", Part::Two) => Some("400"),
            ("Puzzle", Part::One) => Some("30158"),
            ("Puzzle", Part::Two) => Some("36474"),
            _ => unreachable!(),
        }
    }
}
