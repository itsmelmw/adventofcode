use std::{fs, slice::Iter};

use crate::pprint::DayOverview;

pub enum Part {
    One,
    Two,
}
impl Part {
    pub fn iter() -> Iter<'static, Self> {
        static PARTS: [Part; 2] = [Part::One, Part::Two];
        PARTS.iter()
    }
    pub fn as_num(&self) -> usize {
        match self {
            Self::One => 1,
            Self::Two => 2,
        }
    }
}

pub trait Solution {
    fn title(&self) -> &str {
        "Title Unknown"
    }
    fn parse(input: &str) -> Self
    where
        Self: Sized;

    fn solve(&self, part: &Part) -> String {
        match part {
            Part::One => self.solve_part_1(),
            Part::Two => self.solve_part_2(),
        }
    }
    fn solve_part_1(&self) -> String;
    fn solve_part_2(&self) -> String;

    fn solution(&self, _input: &InputDir, _part: &Part) -> Option<&str> {
        None
    }
}

pub struct NoSolution;

impl Solution for NoSolution {
    fn parse(_input: &str) -> Self {
        Self
    }
    fn solve_part_1(&self) -> String {
        0.to_string()
    }
    fn solve_part_2(&self) -> String {
        0.to_string()
    }
}

pub struct InputDir {
    name: String,
    directory: String,
}

impl InputDir {
    pub fn new(name: &str, directory: &str) -> Self {
        Self {
            name: name.to_string(),
            directory: directory.to_string(),
        }
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn directory(&self) -> &String {
        &self.directory
    }
    pub fn read_input(&self, day: usize) -> String {
        let path = format!("src/{}/day{:02}.txt", self.directory, day);
        fs::read_to_string(path).unwrap()
    }
}

#[derive(Clone)]
pub struct InputResult<'a> {
    input_dir: &'a InputDir,
    part_1: Option<String>,
    part_2: Option<String>,
}

impl<'a> InputResult<'a> {
    pub fn new(input_dir: &'a InputDir) -> Self {
        Self {
            input_dir,
            part_1: None,
            part_2: None,
        }
    }
    pub fn input(&self) -> &InputDir {
        self.input_dir
    }
    pub fn result(&'a self, part: &Part) -> &'a Option<String> {
        match part {
            Part::One => &self.part_1,
            Part::Two => &self.part_2,
        }
    }
    pub fn set_result(&mut self, part: &Part, result: String) {
        match part {
            Part::One => self.part_1 = Some(result),
            Part::Two => self.part_2 = Some(result),
        }
    }
}

pub trait Calendar {
    fn year(&self) -> usize;
    fn input_dirs(&self) -> &Vec<InputDir>;
    fn solution(&self, day: usize, input: &str) -> Box<dyn Solution>;
    fn solve_day_pretty(&self, day: usize, debug: bool) {
        let mut input_results = self
            .input_dirs()
            .iter()
            .map(InputResult::new)
            .collect::<Vec<InputResult>>();

        for input_idx in 0..input_results.len() {
            let input = input_results[input_idx].input().read_input(day);
            let solution = self.solution(day, &input);
            for part in Part::iter() {
                let result = solution.solve(part);
                input_results[input_idx].set_result(part, result);

                if !debug {
                    print!("\x1B[2J\x1B[1;1H");
                }
                println!("{}", DayOverview::from(day, &solution, &input_results));
            }
        }
    }
}
