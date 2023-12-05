// https://adventofcode.com/2023/day/4

use std::collections::HashSet;

use crate::solutions::Solution;
use crate::{Input, Part};

type Card = (Vec<usize>, HashSet<usize>);

pub struct Day04 {
    cards: Vec<Card>,
}

impl Solution for Day04 {
    fn title(&self) -> &str {
        "Scratchcards"
    }
    fn parse(input: &str) -> Self {
        let cards = input
            .split('\n')
            .map(|line| {
                let (winning, card) = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
                (
                    winning
                        .split_whitespace()
                        .map(|num| num.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>(),
                    card.split_whitespace()
                        .map(|num| num.parse::<usize>().unwrap())
                        .collect::<HashSet<usize>>(),
                )
            })
            .collect::<Vec<Card>>();
        Self { cards }
    }
    fn solve_part_1(&self) -> String {
        self.cards
            .iter()
            .map(
                |card| match card.0.iter().filter(|num| card.1.contains(num)).count() {
                    0 => 0,
                    n => 2_usize.pow(n as u32 - 1),
                },
            )
            .sum::<usize>()
            .to_string()
    }
    fn solve_part_2(&self) -> String {
        let mut counts = vec![1; self.cards.len()];
        self.cards
            .iter()
            .enumerate()
            .map(|(i, card)| {
                let cnt = card.0.iter().filter(|num| card.1.contains(num)).count();
                (i + 1..i + 1 + cnt).for_each(|c| {
                    if c < self.cards.len() {
                        counts[c] += counts[i]
                    }
                });
                counts[i]
            })
            .sum::<usize>()
            .to_string()
    }
    fn solution(&self, input: &Input, part: &Part) -> Option<&str> {
        match (input, part) {
            (Input::Example, Part::One) => Some("13"),
            (Input::Example, Part::Two) => Some("30"),
            (Input::Puzzle, Part::One) => Some("21821"),
            (Input::Puzzle, Part::Two) => Some("5539496"),
        }
    }
}
