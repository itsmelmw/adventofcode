// https://adventofcode.com/2022/day/8

use crate::solution::{InputType, Day};
use itertools::Itertools;
use std::collections::HashSet;

pub struct Day08 {
    data: Vec<usize>,
    dim: usize,
}

impl<'i> Day<'i> for Day08 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn title(&self) -> &str {
        "Treetop Tree House"
    }

    fn parse(input: &'i str) -> Self {
        // Assume the input is square, so the amount
        // of lines is equal to the dimensions.
        let mut dim = 1;
        let data = input
            .as_bytes()
            .iter()
            .filter_map(|x| match x {
                b'\n' => {
                    dim += 1;
                    None
                }
                _ => Some((x - b'0') as usize),
            })
            .collect();
        Self { data, dim }
    }

    fn solve_part_1(&self) -> Self::Part1Output {
        // Create a hashset of pointers, to check whether we counted a tree before
        let mut found = HashSet::<*const usize>::new();

        // Horizontal
        for y in 0..self.dim {
            let h_iter = self.data.iter().skip(y * self.dim).take(self.dim);
            self.insert_visible(h_iter.clone(), &mut found);
            self.insert_visible(h_iter.rev(), &mut found);
        }

        // Vectical
        for x in 0..self.dim {
            let v_iter = self.data.iter().skip(x).step_by(self.dim);
            self.insert_visible(v_iter.clone(), &mut found);
            self.insert_visible(v_iter.rev(), &mut found);
        }

        found.len()
    }

    fn solve_part_2(&self) -> Self::Part2Output {
        (0..self.dim)
            .cartesian_product(0..self.dim)
            .map(|(x, y)| self.scenic_score(x, y))
            .max()
            .unwrap()
    }

    fn solution(
        &self,
        input_type: crate::solution::InputType,
    ) -> (Option<Self::Part1Output>, Option<Self::Part2Output>) {
        match input_type {
            InputType::Examples => (Some(21), Some(8)),
            InputType::Puzzles => (Some(1801), Some(209880)),
        }
    }
}

impl Day08 {
    fn insert_visible<'a, I>(&self, treeline: I, counted: &mut HashSet<*const usize>)
    where
        I: Iterator<Item = &'a usize>,
    {
        let mut highest = None;
        for tree in treeline {
            if highest.is_none() || tree > highest.unwrap() {
                highest = Some(tree);
                counted.insert(tree as *const usize);
                if let Some(9) = highest {
                    break;
                }
            }
        }
    }

    fn count_visible<'a, I>(&self, treeline: I, init_size: &usize) -> usize
    where
        I: Iterator<Item = &'a usize>,
    {
        let mut count = 0;
        for tree in treeline {
            count += 1;
            if tree >= init_size {
                break;
            }
        }
        count
    }

    fn scenic_score(&self, x: usize, y: usize) -> usize {
        let idx = y * self.dim + x;
        let size = &self.data[idx];

        let to_left = self.data.iter().skip(y * self.dim).take(x).rev();
        let to_right = self.data.iter().skip(idx + 1).take(self.dim - x - 1);
        let to_top = self.data.iter().skip(x).step_by(self.dim).take(y).rev();
        let to_bottom = self
            .data
            .iter()
            .skip(idx + self.dim)
            .step_by(self.dim)
            .take(self.dim - y - 1);

        self.count_visible(to_left, size)
            * self.count_visible(to_right, size)
            * self.count_visible(to_top, size)
            * self.count_visible(to_bottom, size)
    }
}
