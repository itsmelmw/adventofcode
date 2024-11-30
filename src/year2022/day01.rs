// https://adventofcode.com/2022/day/1

use crate::solution::{InputType, Day};

struct Top3 {
    list: [usize; 3],
    min_ind: usize,
}

impl Top3 {
    fn new() -> Self {
        Top3 {
            list: [0, 0, 0],
            min_ind: 0,
        }
    }

    fn sum(&self) -> usize {
        self.list.iter().sum()
    }

    fn update_min(&mut self) {
        self.min_ind = 0;
        for i in [1, 2] {
            if self.list[i] < self.list[self.min_ind] {
                self.min_ind = i;
            }
        }
    }

    fn update(&mut self, new: usize) {
        if new > self.list[self.min_ind] {
            self.list[self.min_ind] = new;
            self.update_min();
        }
    }
}

pub struct Day01 {
    data: Vec<usize>,
}

impl<'i> Day<'i> for Day01 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn title(&self) -> &str {
        "Calorie Counting"
    }

    fn parse(input: &'i str) -> Self {
        let data = input
            .split("\n\n")
            .map(|elf| {
                elf.split('\n')
                    .map(|cal| cal.parse::<usize>().unwrap())
                    .sum()
            })
            .collect::<Vec<usize>>();
        Self { data }
    }

    fn solve_part_1(&self) -> Self::Part1Output {
        *self.data.iter().max().unwrap()
    }

    fn solve_part_2(&self) -> Self::Part2Output {
        let mut top3 = Top3::new();
        for &value in self.data.iter() {
            top3.update(value);
        }
        top3.sum()
    }

    fn solution(
        &self,
        input_type: crate::solution::InputType,
    ) -> (Option<Self::Part1Output>, Option<Self::Part2Output>) {
        match input_type {
            InputType::Examples => (Some(24000), Some(45000)),
            InputType::Puzzles => (Some(70296), Some(205381)),
        }
    }
}
