// https://adventofcode.com/2023/day/15

use aoc_utils::solutions::{InputDir, Part, Solution};

enum Operation<'i> {
    Remove(&'i str),
    Insert(&'i str, usize),
}

pub struct Day15<'i> {
    words: Vec<&'i str>,
    operations: Vec<Operation<'i>>,
}

impl<'i> Solution<'i> for Day15<'i> {
    fn title(&self) -> &str {
        "Lens Library"
    }
    fn parse(input: &'i str) -> Self {
        let words = input.split(',').collect::<Vec<&str>>();
        let operations = words
            .clone()
            .iter()
            .map(|s| {
                if let Some(label) = s.strip_suffix('-') {
                    Operation::Remove(label)
                } else if let Some((label, val)) = s.split_once('=') {
                    Operation::Insert(label, val.parse::<usize>().unwrap())
                } else {
                    panic!()
                }
            })
            .collect::<Vec<Operation>>();
        Self { words, operations }
    }
    fn solve_part_1(&self) -> String {
        self.words
            .iter()
            .map(|s| Self::hash(s))
            .sum::<usize>()
            .to_string()
    }
    fn solve_part_2(&self) -> String {
        let mut map: Vec<Vec<(&str, usize)>> = vec![Vec::new(); 256];
        for op in &self.operations {
            match op {
                Operation::Remove(label) => {
                    let map_vec = &mut map[Self::hash(label)];
                    if let Some(idx) = map_vec.iter().position(|l| l.0 == *label) {
                        map_vec.remove(idx);
                    }
                }
                Operation::Insert(label, val) => {
                    let map_vec = &mut map[Self::hash(label)];
                    match map_vec.iter().position(|l| l.0 == *label) {
                        Some(idx) => map_vec[idx].1 = *val,
                        None => map_vec.push((label, *val)),
                    }
                }
            }
        }
        map.iter()
            .enumerate()
            .flat_map(|(bx, vec)| {
                vec.iter()
                    .enumerate()
                    .map(|(idx, val)| (bx + 1) * (idx + 1) * val.1)
                    .collect::<Vec<usize>>()
            })
            .sum::<usize>()
            .to_string()
    }
    fn answer(&self, input: &InputDir, part: &Part) -> Option<&str> {
        match (input.name().as_str(), part) {
            ("Example", Part::One) => Some("1320"),
            ("Example", Part::Two) => Some("145"),
            ("Puzzle", Part::One) => Some("508498"),
            ("Puzzle", Part::Two) => Some("279116"),
            _ => unreachable!(),
        }
    }
}

impl<'a> Day15<'a> {
    fn hash(word: &'a str) -> usize {
        word.bytes()
            .fold(0u8, |acc, v| acc.wrapping_add(v).wrapping_mul(17)) as usize
    }
}
