// https://adventofcode.com/2023/day/12

use crate::solution::{InputType, Solution};
use std::{collections::HashMap, iter};

type Cache = HashMap<(usize, usize, usize), usize>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Status {
    Operational,
    Damaged,
    Unknown,
}

struct Row {
    springs: Vec<Status>,
    contiguous: Vec<usize>,
}

pub struct Day12 {
    rows: Vec<Row>,
}

impl<'i> Solution<'i> for Day12 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn title(&self) -> &str {
        "Hot Springs"
    }

    fn parse(input: &'i str) -> Self {
        let rows = input
            .split('\n')
            .map(|line| {
                let (springs, contiguous) = line.split_once(' ').unwrap();
                let springs = springs
                    .chars()
                    .map(|c| match c {
                        '.' => Status::Operational,
                        '#' => Status::Damaged,
                        '?' => Status::Unknown,
                        _ => unreachable!(),
                    })
                    .collect::<Vec<Status>>();
                let contiguous = contiguous
                    .split(',')
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                Row {
                    springs,
                    contiguous,
                }
            })
            .collect::<Vec<Row>>();
        Self { rows }
    }

    fn solve_part_1(&self) -> Self::Part1Output {
        self.rows
            .iter()
            .map(|row| {
                let mut cache = HashMap::new();
                self.solve_row(&row.springs, &row.contiguous, 0, &mut cache)
            })
            .sum::<usize>()
    }
    fn solve_part_2(&self) -> Self::Part2Output {
        self.rows
            .iter()
            .map(|row| {
                let springs = iter::repeat(row.springs.clone())
                    .take(5)
                    .collect::<Vec<Vec<Status>>>()
                    .join(&Status::Unknown);
                let contiguous = row.contiguous.repeat(5);
                let mut cache = HashMap::new();
                self.solve_row(&springs, &contiguous, 0, &mut cache)
            })
            .sum::<usize>()
    }

    fn solution(
        &self,
        input_type: crate::solution::InputType,
    ) -> (Option<Self::Part1Output>, Option<Self::Part2Output>) {
        match input_type {
            InputType::Examples => (Some(21), Some(525152)),
            InputType::Puzzles => (Some(7260), Some(1909291258644)),
        }
    }
}

impl Day12 {
    fn solve_row(
        &self,
        springs: &[Status],
        contiguous: &[usize],
        cur_contiguous: usize,
        cache: &mut Cache,
    ) -> usize {
        let cache_key = (springs.len(), contiguous.len(), cur_contiguous);
        if let Some(v) = cache.get(&cache_key) {
            return *v;
        }
        let v = if let Some((curr, tail)) = springs.split_first() {
            match curr {
                Status::Unknown => {
                    self.solve_head(Status::Operational, tail, contiguous, cur_contiguous, cache)
                        + self.solve_head(Status::Damaged, tail, contiguous, cur_contiguous, cache)
                }
                status => self.solve_head(*status, tail, contiguous, cur_contiguous, cache),
            }
        } else {
            (contiguous.is_empty() || (contiguous.len() == 1 && contiguous[0] == cur_contiguous))
                as usize
        };
        cache.insert(cache_key, v);
        v
    }
    fn solve_head(
        &self,
        head: Status,
        tail: &[Status],
        contiguous: &[usize],
        cur_contiguous: usize,
        cache: &mut Cache,
    ) -> usize {
        match head {
            Status::Damaged => {
                if contiguous.is_empty() || cur_contiguous >= contiguous[0] {
                    0
                } else {
                    self.solve_row(tail, contiguous, cur_contiguous + 1, cache)
                }
            }
            Status::Operational => {
                if cur_contiguous != 0 {
                    if cur_contiguous < contiguous[0] {
                        0
                    } else {
                        self.solve_row(tail, &contiguous[1..], 0, cache)
                    }
                } else {
                    self.solve_row(tail, contiguous, 0, cache)
                }
            }
            _ => unreachable!(),
        }
    }
}
