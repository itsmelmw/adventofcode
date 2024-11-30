// https://adventofcode.com/2023/day/14

use std::collections::HashMap;

use crate::{
    grids::Grid,
    solution::{InputType, Solution},
};
use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Rock {
    Empty,
    Round,
    Square,
}

pub struct Day14 {
    grid: Grid<Rock>,
}

impl<'i> Solution<'i> for Day14 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn title(&self) -> &str {
        "Parabolic Reflector Dish"
    }

    fn parse(input: &'i str) -> Self {
        let width = input.find('\n').unwrap();
        let vec = input
            .chars()
            .filter_map(|c| match c {
                '.' => Some(Rock::Empty),
                'O' => Some(Rock::Round),
                '#' => Some(Rock::Square),
                _ => None,
            })
            .collect_vec();
        let grid = Grid::from_vec(vec, width);
        Self { grid }
    }

    fn solve_part_1(&self) -> Self::Part1Output {
        self.grid
            .iter_cols()
            .map(|col| {
                let mut empty_count = 0;
                let mut weight = 0;
                col.enumerate().for_each(|(i, t)| match t {
                    Rock::Empty => empty_count += 1,
                    Rock::Round => weight += (self.grid.height() - i) + empty_count,
                    Rock::Square => empty_count = 0,
                });
                weight
            })
            .sum::<usize>()
    }

    fn solve_part_2(&self) -> Self::Part2Output {
        let mut prev_states = HashMap::new();
        let mut cur_grid = self.grid.clone();

        let mut i = 1000000000;
        while i > 0 {
            if let Some(old_i) = prev_states.get(cur_grid.as_vec()) {
                i %= old_i - i;
            }
            prev_states.insert(cur_grid.as_vec().clone(), i);
            i -= 1;
            // Move north
            let new_vec = self.move_and_flatten(cur_grid.iter_cols());
            cur_grid = Grid::from_vec(new_vec, cur_grid.height()).transposed();

            // Move west
            let new_vec = self.move_and_flatten(cur_grid.iter_rows());
            cur_grid = Grid::from_vec(new_vec, cur_grid.width());

            // Move south
            let new_vec = self.rev_move_and_flatten(cur_grid.iter_cols());
            cur_grid = Grid::from_vec(new_vec, cur_grid.height()).transposed();

            // Move east
            let new_vec = self.rev_move_and_flatten(cur_grid.iter_rows());
            cur_grid = Grid::from_vec(new_vec, cur_grid.width());
        }
        // Calculate weight score
        cur_grid
            .iter_cols()
            .map(|col| {
                col.enumerate()
                    .map(|(i, t)| match t {
                        Rock::Round => self.grid.height() - i,
                        _ => 0,
                    })
                    .sum::<usize>()
            })
            .sum::<usize>()
    }

    fn solution(
        &self,
        input_type: crate::solution::InputType,
    ) -> (Option<Self::Part1Output>, Option<Self::Part2Output>) {
        match input_type {
            InputType::Examples => (Some(136), Some(64)),
            InputType::Puzzles => (Some(109665), Some(96061)),
        }
    }
}

impl Day14 {
    fn move_and_flatten<'a, I, J>(&self, iter: I) -> Vec<Rock>
    where
        I: Iterator<Item = J> + DoubleEndedIterator + ExactSizeIterator,
        J: Iterator<Item = &'a Rock> + DoubleEndedIterator + ExactSizeIterator,
    {
        iter.flat_map(|subiter| self.move_line(subiter))
            .collect::<Vec<Rock>>()
    }

    fn rev_move_and_flatten<'a, I, J>(&self, iter: I) -> Vec<Rock>
    where
        I: Iterator<Item = J> + DoubleEndedIterator + ExactSizeIterator,
        J: Iterator<Item = &'a Rock> + DoubleEndedIterator + ExactSizeIterator,
    {
        iter.flat_map(|subiter| {
            let mut vec = self.move_line(subiter.rev());
            vec.reverse();
            vec
        })
        .collect::<Vec<Rock>>()
    }

    fn move_line<'a, I>(&self, iter: I) -> Vec<Rock>
    where
        I: Iterator<Item = &'a Rock> + ExactSizeIterator,
    {
        let mut vec = Vec::with_capacity(iter.len());
        let mut prev_empty = 0;
        iter.enumerate().for_each(|(i, t)| match t {
            Rock::Empty => vec.push(Rock::Empty),
            Rock::Square => {
                prev_empty = i + 1;
                vec.push(Rock::Square)
            }
            Rock::Round => {
                vec.push(Rock::Empty);
                vec[prev_empty] = Rock::Round;
                prev_empty += 1;
            }
        });
        vec
    }
}
