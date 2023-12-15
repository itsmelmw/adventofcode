mod solutions;

use aoc_utils::solutions::{Calendar, InputDir, Solution};
use solutions::get_solution;
use std::env;

struct Aoc2023 {
    input_dirs: Vec<InputDir>,
}

impl Calendar for Aoc2023 {
    fn year(&self) -> usize {
        2023
    }
    fn input_dirs(&self) -> &Vec<InputDir> {
        &self.input_dirs
    }
    fn solution<'i>(&self, day: usize, input: &'i str) -> Box<dyn Solution<'i> + 'i> {
        get_solution(day, input)
    }
}

fn main() {
    let year = Aoc2023 {
        input_dirs: vec![
            InputDir::new("Example", "src/examples"),
            InputDir::new("Puzzle", "src/inputs"),
        ],
    };
    if let Some(arg) = env::args().nth(1) {
        match arg.parse::<usize>() {
            Ok(day @ 1..=25) => {
                year.solve_day_pretty(day, true);
            }
            _ => {
                println!("Please enter a valid day.");
            }
        };
    } else {
        year.solve_all_pretty();
    }
}
