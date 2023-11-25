// https://adventofcode.com/2022/day/15

use crate::solutions::{InputParser, ProblemSolver};
use crate::utils::IPoint;
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

fn parse(input: &str) -> Vec<(IPoint, IPoint)> {
    return input
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
}

fn solve1(parsed: &Vec<(IPoint, IPoint)>) -> String {
    // Hardcode Y-coordinate if the input is the example.
    let y_coord = if parsed.len() == 14 { 10 } else { 2_000_000 };

    let mut min = 0;
    let mut max = 0;

    // Assume the ranges do not have any holes, which they don't for part 1
    parsed.iter().for_each(|(sensor, beacon)| {
        if let Some((rmin, rmax)) = make_range(sensor, beacon, y_coord) {
            min = min.min(rmin);
            max = max.max(rmax);
        }
    });

    // Answer should be +1 for inclusive range,
    // but -1 because there is a beacon on that Y.
    (max - min).to_string()
}

// There MUST be a more efficient solution for this one, but I'll accept this.
fn solve2(parsed: &Vec<(IPoint, IPoint)>) -> String {
    // Hardcoding example input again
    let y_range = if parsed.len() == 14 { 20 } else { 4_000_000 };

    for y_coord in 0..y_range {
        let mut ranges = vec![];
        parsed.iter().for_each(|(sensor, beacon)| {
            if let Some(range) = make_range(sensor, beacon, y_coord) {
                ranges.push(range);
            }
        });
        if let Some(x_coord) = overlapping(ranges) {
            return (x_coord * 4000000 + y_coord).to_string();
        }
    }
    // Did not find solution
    0.to_string()
}

pub struct Parser;

impl InputParser for Parser {
    type S = Solver;
    fn parse(input: &str) -> Solver {
        let data = parse(input);
        Solver { data }
    }
}

pub struct Solver {
    data: Vec<(IPoint, IPoint)>,
}

impl ProblemSolver for Solver {
    fn solve_part_1(&self) -> String {
        solve1(&self.data)
    }
    fn solve_part_2(&self) -> String {
        solve2(&self.data)
    }
}