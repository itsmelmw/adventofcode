// https://adventofcode.com/2022/day/1

use aoc_utils::solutions::{InputDir, Part, Solution};

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
        return self.list.iter().sum();
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

impl<'i> Solution<'i> for Day01 {
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
    fn solve_part_1(&self) -> String {
        return self.data.iter().max().unwrap().to_string();
    }
    fn solve_part_2(&self) -> String {
        let mut top3 = Top3::new();
        for &value in self.data.iter() {
            top3.update(value);
        }
        top3.sum().to_string()
    }
    fn answer(&self, input: &InputDir, part: &Part) -> Option<&str> {
        match (input.name().as_str(), part) {
            ("Example", Part::One) => Some("24000"),
            ("Example", Part::Two) => Some("45000"),
            ("Puzzle", Part::One) => Some("70296"),
            ("Puzzle", Part::Two) => Some("205381"),
            _ => unreachable!(),
        }
    }
}
