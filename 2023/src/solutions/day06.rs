// https://adventofcode.com/2023/day/6

use crate::solutions::Solution;
use crate::{Input, Part};

type Race = (/*time:*/ f64, /*distance:*/ f64);

pub struct Day06 {
    races: Vec<Race>,
    merged: Race,
}

impl Solution for Day06 {
    fn title(&self) -> &str {
        "Wait For It"
    }
    fn parse(input: &str) -> Self {
        let lines = input
            .split('\n')
            .map(|line| line.split_whitespace().collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>();
        let races = (1..lines[0].len())
            .map(|i| {
                (
                    lines[0][i].parse::<f64>().unwrap(),
                    lines[1][i].parse::<f64>().unwrap(),
                )
            })
            .collect::<Vec<Race>>();
        let merged = (
            lines[0][1..].join("").parse::<f64>().unwrap(),
            lines[1][1..].join("").parse::<f64>().unwrap(),
        );
        Self { races, merged }
    }
    fn solve_part_1(&self) -> String {
        self.races
            .iter()
            .map(|(t, d)| self.solve_quadratic(t, d))
            .product::<usize>()
            .to_string()
    }
    fn solve_part_2(&self) -> String {
        self.solve_quadratic(&self.merged.0, &self.merged.1)
            .to_string()
    }
    fn solution(&self, input: &Input, part: &Part) -> Option<&str> {
        match (input, part) {
            (Input::Example, Part::One) => Some("288"),
            (Input::Puzzle, Part::One) => Some("393120"),
            (Input::Example, Part::Two) => Some("71503"),
            (Input::Puzzle, Part::Two) => Some("36872656"),
        }
    }
}

impl Day06 {
    fn solve_quadratic(&self, time: &f64, distance: &f64) -> usize {
        //  --- Math ---
        //  x * (t - x)  = d
        // -x^2 + tx     = d
        //  x^2 - tx + d = 0
        let sqrt_discr = ((time * time) - 4. * distance).sqrt();
        let x1 = (time - sqrt_discr) / 2.;
        let x2 = (time + sqrt_discr) / 2.;
        (x2.ceil() - x1.floor() - 1.) as usize
    }
}
