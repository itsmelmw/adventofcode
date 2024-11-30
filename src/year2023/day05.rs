// https://adventofcode.com/2023/day/5

use itertools::Itertools;

use crate::solution::{InputType, Solution};

struct Map {
    dst: usize,
    src: usize,
    num: usize,
}

impl Map {
    pub fn contains(&self, val: usize) -> bool {
        val >= self.src && val < self.src + self.num
    }
    pub fn map(&self, val: usize) -> usize {
        val - self.src + self.dst
    }
}

pub struct Day05 {
    seeds: Vec<usize>,
    maps: Vec<Vec<Map>>,
}

impl<'i> Solution<'i> for Day05 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn title(&self) -> &str {
        "If You Give A Seed A Fertilizer"
    }

    fn parse(input: &'i str) -> Self {
        let mut iter = input.split("\n\n");
        let seeds = iter
            .next()
            .unwrap()
            .split(' ')
            .skip(1)
            .map(|num| num.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let maps = iter
            .map(|m| {
                m.split('\n')
                    .skip(1)
                    .map(|line| {
                        let values = line
                            .split(' ')
                            .map(|v| v.parse::<usize>().unwrap())
                            .collect::<Vec<usize>>();
                        Map {
                            dst: values[0],
                            src: values[1],
                            num: values[2],
                        }
                    })
                    .collect::<Vec<Map>>()
            })
            .collect::<Vec<Vec<Map>>>();
        Self { seeds, maps }
    }

    fn solve_part_1(&self) -> Self::Part1Output {
        self.seeds
            .iter()
            .map(|seed| {
                let mut mapped = *seed;
                for map_set in &self.maps {
                    for map in map_set {
                        if map.contains(mapped) {
                            mapped = map.map(mapped);
                            break;
                        }
                    }
                }
                mapped
            })
            .min()
            .unwrap()
    }

    fn solve_part_2(&self) -> Self::Part2Output {
        self.seeds
            .iter()
            .tuples()
            .map(|(&start, &size)| {
                (start..start + size)
                    .map(|seed| {
                        let mut mapped = seed;
                        for map_set in &self.maps {
                            for map in map_set {
                                if map.contains(mapped) {
                                    mapped = map.map(mapped);
                                    break;
                                }
                            }
                        }
                        mapped
                    })
                    .min()
                    .unwrap()
            })
            .min()
            .unwrap()
    }

    fn solution(
        &self,
        input_type: crate::solution::InputType,
    ) -> (Option<Self::Part1Output>, Option<Self::Part2Output>) {
        match input_type {
            InputType::Examples => (Some(35), Some(46)),
            InputType::Puzzles => (Some(309796150), Some(50716416)),
        }
    }
}
