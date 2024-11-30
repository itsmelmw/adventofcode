// https://adventofcode.com/2023/day/6

use crate::solution::{InputType, Solution};

type Race = (/*time:*/ f64, /*distance:*/ f64);

pub struct Day06 {
    races: Vec<Race>,
    merged: Race,
}

impl<'i> Solution<'i> for Day06 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn title(&self) -> &str {
        "Wait For It"
    }

    fn parse(input: &'i str) -> Self {
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

    fn solve_part_1(&self) -> Self::Part1Output {
        self.races
            .iter()
            .map(|(t, d)| self.solve_quadratic(t, d))
            .product::<usize>()
    }

    fn solve_part_2(&self) -> Self::Part2Output {
        self.solve_quadratic(&self.merged.0, &self.merged.1)
    }

    fn solution(
        &self,
        input_type: crate::solution::InputType,
    ) -> (Option<Self::Part1Output>, Option<Self::Part2Output>) {
        match input_type {
            InputType::Examples => (Some(288), Some(71503)),
            InputType::Puzzles => (Some(393120), Some(36872656)),
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
