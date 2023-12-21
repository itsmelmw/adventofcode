use std::{fs, slice::Iter};

use crate::pprint::{DayOverview, Symbol, YearOverview};

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

pub trait Solution<'i> {
    fn title(&self) -> &str {
        "Title Unknown"
    }
    fn parse(input: &'i str) -> Self
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

    fn answer(&self, _input: &InputDir, _part: &Part) -> Option<&str> {
        None
    }
}

pub struct NoSolution;

impl<'i> Solution<'i> for NoSolution {
    fn parse(_input: &'i str) -> Self {
        Self
    }
    fn solve_part_1(&self) -> String {
        0.to_string()
    }
    fn solve_part_2(&self) -> String {
        0.to_string()
    }
}

#[derive(Clone)]
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
        let path = format!("{}/day{:02}.txt", self.directory, day);
        fs::read_to_string(path).unwrap()
    }
}

#[derive(Clone)]
pub struct InputResult<'a> {
    input_dir: &'a InputDir,
    part_1: (Symbol, Option<String>),
    part_2: (Symbol, Option<String>),
}

impl<'a> InputResult<'a> {
    pub fn new(input_dir: &'a InputDir) -> Self {
        Self {
            input_dir,
            part_1: (Symbol::Waiting, None),
            part_2: (Symbol::Waiting, None),
        }
    }
    pub fn input(&self) -> &InputDir {
        self.input_dir
    }
    pub fn result(&'a self, part: &Part) -> &'a (Symbol, Option<String>) {
        match part {
            Part::One => &self.part_1,
            Part::Two => &self.part_2,
        }
    }
    pub fn set_result(&mut self, part: &Part, symbol: Symbol, result: String) {
        match part {
            Part::One => self.part_1 = (symbol, Some(result)),
            Part::Two => self.part_2 = (symbol, Some(result)),
        }
    }
}

pub trait Calendar {
    fn year(&self) -> usize;
    fn input_dirs(&self) -> &Vec<InputDir>;
    fn solution<'i>(&self, day: usize, input: &'i str) -> Box<dyn Solution<'i> + 'i>;
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
                let truth = solution.answer(input_results[input_idx].input(), part);
                input_results[input_idx].set_result(part, Symbol::from_result(&result, truth), result);

                if !debug {
                    print!("\x1B[2J\x1B[1;1H");
                }
                println!(
                    "{}",
                    DayOverview::from(day, solution.title(), &input_results)
                );
            }
        }
    }
    fn solve_all_pretty(&self) {
        let input_results = self
            .input_dirs()
            .iter()
            .map(InputResult::new)
            .collect::<Vec<InputResult>>();
        let mut day_results = (0..25)
            .map(|_| input_results.clone())
            .collect::<Vec<Vec<InputResult>>>();

        for day_idx in 0..day_results.len() {
            let day = day_idx + 1;
            for day_result in &mut day_results[day_idx] {
                let input = day_result.input().read_input(day);
                let solution = self.solution(day, &input);
                for part in Part::iter() {
                    let result = solution.solve(part);
                    let truth = solution.answer(day_result.input(), part);
                    day_result.set_result(part, Symbol::from_result(&result, truth), result);
                }
            }
            print!("\x1B[2J\x1B[1;1H");
            println!("{}", YearOverview::from(self.year(), &day_results));
        }
    }
}
