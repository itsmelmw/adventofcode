// https://adventofcode.com/2022/day/20

use aoc_utils::solutions::{InputDir, Part, Solution};

pub struct Day20 {
    data: Vec<isize>,
}

impl Solution for Day20 {
    fn title(&self) -> &str {
        "Grove Positioning System"
    }
    fn parse(input: &str) -> Self {
        let data = input
            .split('\n')
            .map(|line| line.parse().unwrap())
            .collect();
        Self { data }
    }
    fn solve_part_1(&self) -> String {
        self.mix_and_sum(&self.data, 1).to_string()
    }
    fn solve_part_2(&self) -> String {
        let decrypted = self
            .data
            .iter()
            .map(|i| i * 811589153)
            .collect::<Vec<isize>>();
        self.mix_and_sum(&decrypted, 10).to_string()
    }
    fn solution(&self, input: &InputDir, part: &Part) -> Option<&str> {
        match (input.name().as_str(), part) {
            ("Example", Part::One) => Some("3"),
            ("Example", Part::Two) => Some("1623178306"),
            ("Puzzle", Part::One) => Some("13183"),
            ("Puzzle", Part::Two) => Some("6676132372578"),
            _ => unreachable!(),
        }
    }
}

impl Day20 {
    fn mix_and_sum(&self, list: &Vec<isize>, iters: usize) -> isize {
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
