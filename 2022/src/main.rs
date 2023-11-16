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
                let vec = vec![
                    Output {
                        title: "Example",
                        result: get_solution("examples", d),
                        truth: EXAMPLE_TRUTHS[d - 1],
                    },
                    Output {
                        title: "Puzzle",
                        result: get_solution("input", d),
                        truth: PUZZLE_TRUTHS[d - 1],
                    },
                ];

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
    fs::read_to_string(path).ok()
}

fn get_solution(dir: &str, day: usize) -> Result<(String, String), &str> {
    match read_input(dir, day) {
        Some(input) => match day {
            1 => Ok(day01::solve(&input)),
            2 => Ok(day02::solve(&input)),
            3 => Ok(day03::solve(&input)),
            4 => Ok(day04::solve(&input)),
            5 => Ok(day05::solve(&input)),
            6 => Ok(day06::solve(&input)),
            7 => Ok(day07::solve(&input)),
            8 => Ok(day08::solve(&input)),
            9 => Ok(day09::solve(&input)),
            10 => Ok(day10::solve(&input)),
            11 => Ok(day11::solve(&input)),
            12 => Ok(day12::solve(&input)),
            13 => Ok(day13::solve(&input)),
            14 => Ok(day14::solve(&input)),
            15 => Ok(day15::solve(&input)),
            16 => Ok(day16::solve(&input)),
            17 => Ok(day17::solve(&input)),
            18 => Ok(day18::solve(&input)),
            19 => Ok(day19::solve(&input)),
            20 => Ok(day20::solve(&input)),
            _ => Err("No solution"),
        },
        None => Err("File missing"),
    }
}
