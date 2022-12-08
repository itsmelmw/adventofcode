use std::env;
use std::fs;

mod pprint;
mod solutions;
mod truths;

use pprint::{pprint_solutions, Output};
use solutions::*;
use truths::{EXAMPLE_TRUTHS, PUZZLE_TRUTHS};

fn main() {
    if let Some(arg) = env::args().nth(1) {
        let day = arg.parse::<usize>();
        match day {
            Ok(d @ 1..=25) => {
                let mut vec = Vec::<Output>::new();
                vec.push(Output {
                    title: "Example",
                    result: get_solution("examples", d),
                    truth: EXAMPLE_TRUTHS[d - 1],
                });
                vec.push(Output {
                    title: "Puzzle",
                    result: get_solution("input", d),
                    truth: PUZZLE_TRUTHS[d - 1],
                });
                pprint_solutions(d, vec);
            }
            _ => {
                println!("Please give a valid day.");
            }
        };
    }
}

fn read_input(dir: &str, day: usize) -> Option<String> {
    let path = format!("src/{}/day{:02}.txt", dir, day);
    return fs::read_to_string(path).ok();
}

fn get_solution(dir: &str, day: usize) -> Result<(String, String), &str> {
    return match read_input(dir, day) {
        Some(input) => match day {
            1 => Ok(day01::solve(&input)),
            2 => Ok(day02::solve(&input)),
            3 => Ok(day03::solve(&input)),
            4 => Ok(day04::solve(&input)),
            5 => Ok(day05::solve(&input)),
            6 => Ok(day06::solve(&input)),
            7 => Ok(day07::solve(&input)),
            8 => Ok(day08::solve(&input)),
            _ => Err("No solution"),
        },
        None => Err("File missing"),
    };
}
