// https://adventofcode.com/2022/day/11

use itertools::Itertools;
use std::collections::VecDeque;

use aoc_utils::solutions::{InputDir, Part, Solution};

#[derive(Clone)]
enum Val {
    Num(usize),
    Old,
}
#[derive(Clone)]
enum Op {
    Add(Val),
    Mul(Val),
}

#[derive(Clone)]
struct Monkey {
    items: VecDeque<usize>,
    op: Op,
    test: usize,
    ttrue: usize,
    tfalse: usize,
    inspected: usize,
}

impl Monkey {
    fn new(items: VecDeque<usize>, op: Op, test: usize, ttrue: usize, tfalse: usize) -> Self {
        Self {
            items,
            op,
            test,
            ttrue,
            tfalse,
            inspected: 0,
        }
    }

    fn inspect_item(&mut self) -> usize {
        self.inspected += 1;
        let item = self.items.pop_front().unwrap();
        match self.op {
            Op::Add(Val::Old) => item + item,
            Op::Add(Val::Num(i)) => item + i,
            Op::Mul(Val::Old) => item * item,
            Op::Mul(Val::Num(i)) => item * i,
        }
    }

    fn throw_to(&self, item: usize) -> usize {
        if item % self.test == 0 {
            return self.ttrue;
        }
        self.tfalse
    }
}

pub struct Day11 {
    monkeys: Vec<Monkey>,
}

impl<'i> Solution<'i> for Day11 {
    fn title(&self) -> &str {
        "Monkey in the Middle"
    }
    fn parse(input: &'i str) -> Self {
        let mut monkeys = Vec::new();
        for text in input.split("\n\n") {
            let monkey = text.split('\n').collect::<Vec<&str>>();
            let items = monkey[1]
                .split_at(18)
                .1
                .split(", ")
                .map(|item| item.parse::<usize>().unwrap())
                .collect::<VecDeque<usize>>();
            let operation = match monkey[2].split_at(23).1.split(' ').collect::<Vec<&str>>()[..] {
                ["+", "old"] => Op::Add(Val::Old),
                ["+", num] => Op::Add(Val::Num(num.parse::<usize>().unwrap())),
                ["*", "old"] => Op::Mul(Val::Old),
                ["*", num] => Op::Mul(Val::Num(num.parse::<usize>().unwrap())),
                _ => unreachable!(),
            };
            let test = monkey[3].split_at(21).1.parse::<usize>().unwrap();
            let ttrue = monkey[4].split_at(29).1.parse::<usize>().unwrap();
            let tfalse = monkey[5].split_at(30).1.parse::<usize>().unwrap();
            monkeys.push(Monkey::new(items, operation, test, ttrue, tfalse));
        }
        Self { monkeys }
    }
    fn solve_part_1(&self) -> String {
        let monkeys = &mut self.monkeys.clone();
        for _ in 0..20 {
            for idx in 0..monkeys.len() {
                while !monkeys[idx].items.is_empty() {
                    let mut item = monkeys[idx].inspect_item();
                    item /= 3;
                    let to = monkeys[idx].throw_to(item);
                    monkeys[to].items.push_back(item);
                }
            }
        }
        self.monkey_business(monkeys).to_string()
    }
    fn solve_part_2(&self) -> String {
        let monkeys = &mut self.monkeys.clone();
        let cmul = self
            .monkeys
            .iter()
            .map(|monkey| monkey.test)
            .product::<usize>();
        for _ in 0..10000 {
            for idx in 0..monkeys.len() {
                while !monkeys[idx].items.is_empty() {
                    let mut item = monkeys[idx].inspect_item();
                    item %= cmul;
                    let to = monkeys[idx].throw_to(item);
                    monkeys[to].items.push_back(item);
                }
            }
        }
        self.monkey_business(monkeys).to_string()
    }
    fn answer(&self, input: &InputDir, part: &Part) -> Option<&str> {
        match (input.name().as_str(), part) {
            ("Example", Part::One) => Some("10605"),
            ("Example", Part::Two) => Some("2713310158"),
            ("Puzzle", Part::One) => Some("113220"),
            ("Puzzle", Part::Two) => Some("30599555965"),
            _ => unreachable!(),
        }
    }
}

impl Day11 {
    fn monkey_business(&self, monkeys: &[Monkey]) -> usize {
        monkeys
            .iter()
            .map(|monkey| monkey.inspected)
            .sorted()
            .rev()
            .take(2)
            .product::<usize>()
    }
}
