// https://adventofcode.com/2023/day/7

use crate::solution::{InputType, Day};
use itertools::Itertools;
use std::{cmp::Ordering, collections::HashMap};

trait OrdPart2 {
    fn cmp_part_2(&self, other: &Self) -> Ordering;
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from_char(c: char) -> Self {
        match c {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::Jack,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => unreachable!(),
        }
    }
}

impl OrdPart2 for Card {
    fn cmp_part_2(&self, other: &Self) -> Ordering {
        if let Card::Jack = self {
            if let Card::Jack = other {
                Ordering::Equal
            } else {
                Ordering::Less
            }
        } else if let Card::Jack = other {
            Ordering::Greater
        } else {
            self.cmp(other)
        }
    }
}

impl OrdPart2 for Vec<Card> {
    fn cmp_part_2(&self, other: &Self) -> Ordering {
        self.iter()
            .zip(other)
            .find_map(|(s, o)| match s.cmp_part_2(o) {
                Ordering::Equal => None,
                ord => Some(ord),
            })
            .unwrap_or(Ordering::Equal)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn from_counts(counts: &HashMap<Card, usize>) -> Self {
        match counts.values().max().unwrap() {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 if counts.values().contains(&2) => HandType::FullHouse,
            3 => HandType::ThreeOfAKind,
            2 if counts.values().filter(|&&v| v == 2).count() == 2 => HandType::TwoPair,
            2 => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    hand_type_2: HandType,
    bet: usize,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards.eq(&other.cards) && self.hand_type.eq(&other.hand_type)
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            ord => ord,
        }
    }
}

impl OrdPart2 for Hand {
    fn cmp_part_2(&self, other: &Self) -> Ordering {
        match self.hand_type_2.cmp(&other.hand_type_2) {
            Ordering::Equal => self.cards.cmp_part_2(&other.cards),
            ord => ord,
        }
    }
}

pub struct Day07 {
    hands: Vec<Hand>,
}

impl<'i> Day<'i> for Day07 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn title(&self) -> &str {
        "Camel Cards"
    }

    fn parse(input: &'i str) -> Self {
        let hands = input
            .split('\n')
            .map(|line| {
                let (cards, bet) = line.split_once(' ').unwrap();
                let cards = cards.chars().map(Card::from_char).collect::<Vec<Card>>();
                let mut counts = cards.clone().into_iter().counts();
                let hand_type = HandType::from_counts(&counts);

                if let Some(jokers) = counts.remove(&Card::Jack) {
                    let max_card = counts
                        .iter()
                        .max_by_key(|e| e.1)
                        .map(|e| *e.0)
                        .unwrap_or(Card::Jack);
                    let max_card_num = counts.get(&max_card).unwrap_or(&0);
                    counts.insert(max_card, max_card_num + jokers);
                }
                let hand_type_2 = HandType::from_counts(&counts);

                let bet = bet.parse::<usize>().unwrap();
                Hand {
                    cards,
                    hand_type,
                    hand_type_2,
                    bet,
                }
            })
            .collect::<Vec<Hand>>();
        Self { hands }
    }

    fn solve_part_1(&self) -> Self::Part1Output {
        self.hands
            .iter()
            .sorted()
            .enumerate()
            .map(|(i, h)| h.bet * (i + 1))
            .sum::<usize>()
    }

    fn solve_part_2(&self) -> Self::Part2Output {
        self.hands
            .iter()
            .sorted_by(|x, y| x.cmp_part_2(y))
            .enumerate()
            .map(|(i, h)| h.bet * (i + 1))
            .sum::<usize>()
    }

    fn solution(
        &self,
        input_type: crate::solution::InputType,
    ) -> (Option<Self::Part1Output>, Option<Self::Part2Output>) {
        match input_type {
            InputType::Examples => (Some(6440), Some(5905)),
            InputType::Puzzles => (Some(251806792), Some(252113488)),
        }
    }
}
