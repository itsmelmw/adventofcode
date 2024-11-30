mod day01;
use crate::solution::{Day, InputType, NoSolution, Year};

pub struct Year2024;

impl Year for Year2024 {
    fn year(&self) -> usize {
        2024
    }
    fn solve(&self, day: usize, input_type: InputType) {
        let input = self.get_input(day, input_type);
        match day {
            1 => day01::Day01::parse(&input).solve(),
            _ => unreachable!(),
        }
    }
}
