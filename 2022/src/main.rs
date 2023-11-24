mod pprint;
mod solutions;
mod truths;
mod utils;

use pprint::{DayTable, YearTable};
use solutions::get_solver;
use std::{env, fs, slice::Iter};
use truths::get_truth;

const YEAR: usize = 2022;

#[derive(Default)]
pub enum CheckResult {
    #[default]
    Waiting,
    Correct(String),
    Wrong(String),
    Unknown(String),
}

impl CheckResult {
    pub fn symbol(&self) -> &'static str {
        match self {
            CheckResult::Waiting => "\x1b[36m…\x1b[0m",
            CheckResult::Correct(_) => "\x1b[32m✔\x1b[0m",
            CheckResult::Wrong(_) => "\x1b[31m✘\x1b[0m",
            CheckResult::Unknown(_) => "\x1b[33m?\x1b[0m",
        }
    }

    pub fn result(&self) -> String {
        match self {
            CheckResult::Waiting => "Waiting...".to_string(),
            CheckResult::Correct(s) => s.to_string(),
            CheckResult::Wrong(s) => s.to_string(),
            CheckResult::Unknown(s) => s.to_string(),
        }
    }
}

pub enum Part {
    One,
    Two,
}
impl Part {
    pub fn iter() -> Iter<'static, Self> {
        static PARTS: [Part; 2] = [Part::One, Part::Two];
        PARTS.iter()
    }
}

pub enum Input {
    Example,
    Puzzle,
}
impl Input {
    pub fn iter() -> Iter<'static, Self> {
        static INPUTS: [Input; 2] = [Input::Example, Input::Puzzle];
        INPUTS.iter()
    }
    pub fn name(&self) -> &str {
        match self {
            Input::Example => "Example",
            Input::Puzzle => "Puzzle",
        }
    }
    pub fn directory(&self) -> &str {
        match self {
            Input::Example => "examples",
            Input::Puzzle => "inputs",
        }
    }
    pub fn read_for_day(&self, day: usize) -> String {
        let path = format!("src/{}/day{:02}.txt", self.directory(), day);
        fs::read_to_string(path).unwrap()
    }
}

fn main() {
    if let Some(arg) = env::args().nth(1) {
        let day = arg.parse::<usize>();
        match day {
            Ok(d @ 1..=25) => {
                let mut table = DayTable::new(d);

                for (i, input) in Input::iter().enumerate() {
                    let solver = get_solver(d, &input.read_for_day(d));
                    for part in Part::iter() {
                        let solution = solver.solve(part);
                        let truth = get_truth(d, input, part);
                        let result = get_result(solution, truth);

                        table.set_result(i, part, result);
                    }
                }
            }
            _ => {
                println!("Please enter a valid day.");
            }
        };
    } else {
        let mut table = YearTable::new(YEAR);
        'day: for d in 1..=25 {
            for input in Input::iter() {
                let solver = get_solver(d, &input.read_for_day(d));
                for part in Part::iter() {
                    let solution = solver.solve(part);
                    let truth = get_truth(d, input, part);
                    match get_result(solution, truth) {
                        CheckResult::Correct(_) => (),
                        res => {
                            table.set_result(d, res);
                            continue 'day;
                        }
                    }
                }
            }
            table.set_result(d, CheckResult::Correct("Passed!".to_string()));
        }
    }
}

fn get_result(solution: String, truth: Option<&str>) -> CheckResult {
    match (solution, truth) {
        (s, None) => CheckResult::Unknown(s),
        (s, Some(t)) if s == t => CheckResult::Correct(s),
        (s, _) => CheckResult::Wrong(s),
    }
}
