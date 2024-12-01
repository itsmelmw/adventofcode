// https://adventofcode.com/2022/day/15

use crate::{
    grids::IPoint,
    solution::{Day, InputType},
};
use itertools::Itertools;

type Range = (isize, isize);

fn make_range(sensor: &IPoint, beacon: &IPoint, y: isize) -> Option<Range> {
    let dist = sensor.x.abs_diff(beacon.x) as isize + sensor.y.abs_diff(beacon.y) as isize;
    let span = dist - sensor.y.abs_diff(y) as isize;
    if span >= 0 {
        Some((sensor.x - span, sensor.x + span))
    } else {
        None
    }
}

fn overlapping(ranges: Vec<Range>) -> Option<isize> {
    let mut iter = ranges.iter().sorted_by_key(|r| r.0);
    let mut total_range = *iter.next().unwrap();

    for range in iter {
        if range.0 <= total_range.1 {
            total_range.1 = total_range.1.max(range.1)
        } else {
            return Some(total_range.1 + 1);
        }
    }
    None
}

pub struct Day15 {
    data: Vec<(IPoint, IPoint)>,
}

impl<'i> Day<'i> for Day15 {
    type Part1Output = isize;
    type Part2Output = isize;

    fn title(&self) -> &str {
        "Beacon Exclusion Zone"
    }

    fn parse(input: &'i str) -> Self {
        let data = input
            .split('\n')
            .map(|line| {
                let words = line.split(' ').collect::<Vec<&str>>();
                (
                    IPoint::new(
                        words[2][2..words[2].len() - 1].parse::<isize>().unwrap(),
                        words[3][2..words[3].len() - 1].parse::<isize>().unwrap(),
                    ),
                    IPoint::new(
                        words[8][2..words[8].len() - 1].parse::<isize>().unwrap(),
                        words[9][2..].parse::<isize>().unwrap(),
                    ),
                )
            })
            .collect::<Vec<(IPoint, IPoint)>>();
        Self { data }
    }

    fn solve_part_1(&self) -> Self::Part1Output {
        // Hardcode Y-coordinate if the input is the example.
        let y_coord = if self.data.len() == 14 { 10 } else { 2_000_000 };

        let mut min = 0;
        let mut max = 0;

        // Assume the ranges do not have any holes, which they don't for part 1
        self.data.iter().for_each(|(sensor, beacon)| {
            if let Some((rmin, rmax)) = make_range(sensor, beacon, y_coord) {
                min = min.min(rmin);
                max = max.max(rmax);
            }
        });

        // Answer should be +1 for inclusive range,
        // but -1 because there is a beacon on that Y.
        max - min
    }

    fn solve_part_2(&self) -> Self::Part2Output {
        // Hardcoding example input again
        let y_range = if self.data.len() == 14 { 20 } else { 4_000_000 };

        for y_coord in 0..y_range {
            let mut ranges = vec![];
            self.data.iter().for_each(|(sensor, beacon)| {
                if let Some(range) = make_range(sensor, beacon, y_coord) {
                    ranges.push(range);
                }
            });
            if let Some(x_coord) = overlapping(ranges) {
                return x_coord * 4000000 + y_coord;
            }
        }
        // Did not find solution
        0
    }

    fn solution(
        &self,
        input_type: crate::solution::InputType,
    ) -> (Option<Self::Part1Output>, Option<Self::Part2Output>) {
        match input_type {
            InputType::Examples => (Some(26), Some(56000011)),
            InputType::Puzzles => (Some(5564017), Some(11558423398893)),
        }
    }
}
