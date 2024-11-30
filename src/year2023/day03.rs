// https://adventofcode.com/2023/day/3

use crate::{
    grids::IPoint,
    solution::{InputType, Day},
};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Hash)]
struct Number {
    value: usize,
    loc: IPoint,
    length: usize,
}

pub struct Day03 {
    numbers: Vec<Number>,
    symbols: HashSet<IPoint>,
    gears: Vec<IPoint>,
}

impl<'i> Day<'i> for Day03 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn title(&self) -> &str {
        "Gear Ratios"
    }

    fn parse(input: &'i str) -> Self {
        let mut numbers = Vec::new();
        let mut symbols = HashSet::new();
        let mut gears = Vec::new();
        input.split('\n').enumerate().for_each(|(y, line)| {
            let mut char_iter = line.chars().enumerate().peekable();
            while let Some((x, c)) = char_iter.next() {
                match c {
                    '.' => continue,
                    d if d.is_ascii_digit() => {
                        let root_point = IPoint::new(x as isize, y as isize);
                        let mut value = d.to_string();
                        while let Some((_, c)) = char_iter.peek() {
                            if !c.is_ascii_digit() {
                                break;
                            }
                            value.push(char_iter.next().unwrap().1);
                        }
                        numbers.push(Number {
                            value: value.parse::<usize>().unwrap(),
                            loc: root_point,
                            length: value.len(),
                        });
                    }
                    s => {
                        if s == '*' {
                            gears.push(IPoint::new(x as isize, y as isize));
                        }
                        symbols.insert(IPoint::new(x as isize, y as isize));
                    }
                }
            }
        });
        Self {
            numbers,
            symbols,
            gears,
        }
    }

    fn solve_part_1(&self) -> usize {
        self.numbers
            .iter()
            .filter_map(|n| {
                for i in 0..n.length {
                    if (n.loc + IPoint::new(i as isize, 0))
                        .neighbors_8()
                        .iter()
                        .any(|p| self.symbols.contains(p))
                    {
                        return Some(n.value);
                    }
                }
                None
            })
            .sum::<usize>()
    }

    fn solve_part_2(&self) -> usize {
        let mut number_map = HashMap::new();
        for n in &self.numbers {
            for i in 0..n.length {
                number_map.insert(n.loc + IPoint::new(i as isize, 0), n);
            }
        }
        self.gears
            .iter()
            .filter_map(|g| {
                let nums = g
                    .neighbors_8()
                    .iter()
                    .filter_map(|neighbor| number_map.get(neighbor).copied())
                    .unique()
                    .collect::<Vec<&Number>>();
                if nums.len() > 1 {
                    Some(nums.iter().map(|n| n.value).product::<usize>())
                } else {
                    None
                }
            })
            .sum::<usize>()
    }

    fn solution(
        &self,
        input_type: crate::solution::InputType,
    ) -> (Option<Self::Part1Output>, Option<Self::Part2Output>) {
        match input_type {
            InputType::Examples => (Some(4361), Some(467835)),
            InputType::Puzzles => (Some(544664), Some(84495585)),
        }
    }
}
