// https://adventofcode.com/2022/day/20

use crate::solution::{InputType, Solution};

pub struct Day20 {
    data: Vec<isize>,
}

impl<'i> Solution<'i> for Day20 {
    type Part1Output = isize;
    type Part2Output = isize;

    fn title(&self) -> &str {
        "Grove Positioning System"
    }

    fn parse(input: &'i str) -> Self {
        let data = input
            .split('\n')
            .map(|line| line.parse().unwrap())
            .collect();
        Self { data }
    }

    fn solve_part_1(&self) -> Self::Part1Output {
        self.mix_and_sum(&self.data, 1)
    }

    fn solve_part_2(&self) -> Self::Part2Output {
        let decrypted = self
            .data
            .iter()
            .map(|i| i * 811589153)
            .collect::<Vec<isize>>();
        self.mix_and_sum(&decrypted, 10)
    }

    fn solution(
        &self,
        input_type: crate::solution::InputType,
    ) -> (Option<Self::Part1Output>, Option<Self::Part2Output>) {
        match input_type {
            InputType::Examples => (Some(3), Some(1623178306)),
            InputType::Puzzles => (Some(13183), Some(6676132372578)),
        }
    }
}

impl Day20 {
    fn mix_and_sum(&self, list: &[isize], iters: usize) -> isize {
        let mut mixed = list.iter().enumerate().collect::<Vec<(usize, &isize)>>();
        for _ in 0..iters {
            for idx in 0..list.len() {
                let num = list[idx];
                let cur_idx = mixed.iter().position(|(i, _)| *i == idx).unwrap();
                let new_idx =
                    (cur_idx as isize + num).rem_euclid((list.len() - 1) as isize) as usize;

                let val = mixed.remove(cur_idx);
                mixed.insert(new_idx, val);
            }
        }

        let zero_idx = mixed.iter().position(|(_, &v)| v == 0).unwrap();
        [1000, 2000, 3000]
            .iter()
            .map(|i| {
                let idx = (zero_idx as isize + i).rem_euclid(list.len() as isize) as usize;
                *mixed[idx].1
            })
            .sum::<isize>()
    }
}
