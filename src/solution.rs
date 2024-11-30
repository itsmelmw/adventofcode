use std::{fmt::Display, fs};

pub trait Day<'i> {
    type Part1Output: Display;
    type Part2Output: Display;

    fn title(&self) -> &str {
        "Title Unknown"
    }

    fn parse(input: &'i str) -> Self
    where
        Self: Sized;

    fn solve_part_1(&self) -> Self::Part1Output;
    fn solve_part_2(&self) -> Self::Part2Output;

    fn solution(
        &self,
        input_type: InputType,
    ) -> (Option<Self::Part1Output>, Option<Self::Part2Output>) {
        (None, None)
    }

    fn solve(&self) {
        println!("Title: {}", self.title());
        println!("Part 1: {}", self.solve_part_1());
        println!("Part 2: {}", self.solve_part_2());
    }
}

pub struct NoSolution;

impl<'i> Day<'i> for NoSolution {
    type Part1Output = usize;
    type Part2Output = usize;

    fn parse(_input: &'i str) -> Self {
        Self
    }
    fn solve_part_1(&self) -> Self::Part1Output {
        0
    }
    fn solve_part_2(&self) -> Self::Part2Output {
        0
    }
}

pub enum InputType {
    Examples,
    Puzzles,
}

impl InputType {
    fn dir(&self) -> &str {
        match self {
            Self::Examples => "examples",
            Self::Puzzles => "puzzles",
        }
    }
}

pub trait Year {
    fn year(&self) -> usize;
    fn solve(&self, day: usize, input_type: InputType);
    fn get_input(&self, day: usize, input_type: InputType) -> String {
        let path = format!(
            "inputs/{}/{}/day{:02}.txt",
            self.year(),
            input_type.dir(),
            day,
        );
        fs::read_to_string(path).expect("Input file could not be read")
    }
}
