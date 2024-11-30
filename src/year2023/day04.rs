// https://adventofcode.com/2023/day/4

use crate::solution::{InputType, Day};
use std::collections::HashSet;

type Card = (Vec<usize>, HashSet<usize>);

pub struct Day04 {
    cards: Vec<Card>,
}

impl<'i> Day<'i> for Day04 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn title(&self) -> &str {
        "Scratchcards"
    }

    fn parse(input: &'i str) -> Self {
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

    fn solve_part_1(&self) -> Self::Part1Output {
        self.cards
            .iter()
            .map(
                |card| match card.0.iter().filter(|num| card.1.contains(num)).count() {
                    0 => 0,
                    n => 2_usize.pow(n as u32 - 1),
                },
            )
            .sum::<usize>()
    }

    fn solve_part_2(&self) -> Self::Part2Output {
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
    }

    fn solution(
        &self,
        input_type: crate::solution::InputType,
    ) -> (Option<Self::Part1Output>, Option<Self::Part2Output>) {
        match input_type {
            InputType::Examples => (Some(13), Some(30)),
            InputType::Puzzles => (Some(21821), Some(5539496)),
        }
    }
}
