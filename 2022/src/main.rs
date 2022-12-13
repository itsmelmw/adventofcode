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
            01 => Ok(day01::solve(&input)),
            02 => Ok(day02::solve(&input)),
            03 => Ok(day03::solve(&input)),
            04 => Ok(day04::solve(&input)),
            05 => Ok(day05::solve(&input)),
            06 => Ok(day06::solve(&input)),
            07 => Ok(day07::solve(&input)),
            08 => Ok(day08::solve(&input)),
            09 => Ok(day09::solve(&input)),
            10 => Ok(day10::solve(&input)),
            11 => Ok(day11::solve(&input)),
            12 => Ok(day12::solve(&input)),
            13 => Ok(day13::solve(&input)),
            _ => Err("No solution"),
        },
        None => Err("File missing"),
    };
}
