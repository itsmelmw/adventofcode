mod solutions;

use aoc_utils::solutions::{Calendar, InputDir, Solution};
use solutions::get_solution;
use std::env;

struct Aoc2022 {
    input_dirs: Vec<InputDir>,
}

impl Calendar for Aoc2022 {
    fn year(&self) -> usize {
        2022
    }
    fn input_dirs(&self) -> &Vec<InputDir> {
        &self.input_dirs
    }
    fn solution(&self, day: usize, input: &str) -> Box<dyn Solution> {
        get_solution(day, input)
    }
}

fn main() {
    let year = Aoc2022 {
        input_dirs: vec![
            InputDir::new("Example", "examples"),
            InputDir::new("Puzzle", "inputs"),
        ],
    };
    if let Some(arg) = env::args().nth(1) {
        match arg.parse::<usize>() {
            Ok(day @ 1..=25) => {
                year.solve_day_pretty(day, false);
            }
            _ => {
                println!("Please enter a valid day.");
            }
        };
    }
}
