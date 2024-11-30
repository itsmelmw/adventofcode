// https://adventofcode.com/2023/day/19

use crate::solution::{InputType, Solution};
use itertools::Itertools;
use std::collections::HashMap;

enum Category {
    X,
    M,
    A,
    S,
}

impl Category {
    fn from_str(s: &str) -> Self {
        match s {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            _ => unreachable!(),
        }
    }
    fn eval(&self, thing: &Thing) -> usize {
        match self {
            Self::X => thing.x,
            Self::M => thing.m,
            Self::A => thing.a,
            Self::S => thing.s,
        }
    }
}

enum Statement {
    IfThen(Condition, Consequence),
    Else(Consequence),
}

impl Statement {
    fn from_str(s: &str) -> Self {
        match s.split_once(':') {
            Some((cnd, cns)) => Self::IfThen(Condition::from_str(cnd), Consequence::from_str(cns)),
            None => Self::Else(Consequence::from_str(s)),
        }
    }
    fn eval(&self, flows: &HashMap<String, Statements>, thing: &Thing) -> Option<bool> {
        match self {
            Self::IfThen(cnd, cns) => {
                if cnd.eval(thing) {
                    Some(cns.eval(flows, thing))
                } else {
                    None
                }
            }
            Self::Else(cns) => Some(cns.eval(flows, thing)),
        }
    }
}

enum Condition {
    LessThan(Category, usize),
    GreaterThan(Category, usize),
}

impl Condition {
    fn from_str(s: &str) -> Self {
        match s.split_once('<') {
            Some((cat, val)) => {
                Self::LessThan(Category::from_str(cat), val.parse::<usize>().unwrap())
            }
            None => {
                let (cat, val) = s.split_once('>').unwrap();
                Self::GreaterThan(Category::from_str(cat), val.parse::<usize>().unwrap())
            }
        }
    }
    fn eval(&self, thing: &Thing) -> bool {
        match self {
            Self::LessThan(cat, val) => cat.eval(thing) < *val,
            Self::GreaterThan(cat, val) => cat.eval(thing) > *val,
        }
    }
}

enum Consequence {
    Accept,
    Reject,
    Jump(String),
}

impl Consequence {
    fn from_str(s: &str) -> Self {
        match s {
            "A" => Self::Accept,
            "R" => Self::Reject,
            label => Self::Jump(label.to_string()),
        }
    }
    fn eval(&self, flows: &HashMap<String, Statements>, thing: &Thing) -> bool {
        match self {
            Self::Accept => true,
            Self::Reject => false,
            Self::Jump(label) => flows[label].eval(flows, thing),
        }
    }
}

struct Thing {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

#[derive(Clone)]
struct ThingRanges {
    x: Vec<usize>,
    m: Vec<usize>,
    a: Vec<usize>,
    s: Vec<usize>,
}

struct Statements(Vec<Statement>);

impl Statements {
    fn eval(&self, flows: &HashMap<String, Statements>, thing: &Thing) -> bool {
        for stmt in &self.0 {
            if let Some(res) = stmt.eval(flows, thing) {
                return res;
            }
        }
        panic!();
    }
}

pub struct Day19 {
    flows: HashMap<String, Statements>,
    things: Vec<Thing>,
}

impl<'i> Solution<'i> for Day19 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn title(&self) -> &str {
        "Aplenty"
    }

    fn parse(input: &'i str) -> Self {
        let (flows, things) = input.split_once("\n\n").unwrap();
        let flows = flows
            .split('\n')
            .map(|line| {
                let (label, flow) = line.split_once('{').unwrap();
                let flow = flow[..flow.len() - 1]
                    .split(',')
                    .map(Statement::from_str)
                    .collect::<Vec<Statement>>();
                (label.to_string(), Statements(flow))
            })
            .collect::<HashMap<String, Statements>>();
        let things = things
            .split('\n')
            .map(|thing| {
                let vals = thing[1..thing.len() - 1]
                    .split(',')
                    .map(|cat| cat.split_once('=').unwrap().1.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                Thing {
                    x: vals[0],
                    m: vals[1],
                    a: vals[2],
                    s: vals[3],
                }
            })
            .collect::<Vec<Thing>>();
        Self { flows, things }
    }

    fn solve_part_1(&self) -> Self::Part1Output {
        self.things
            .iter()
            .filter_map(|thing| {
                if self.flows.get("in").unwrap().eval(&self.flows, thing) {
                    Some(thing.x + thing.m + thing.a + thing.s)
                } else {
                    None
                }
            })
            .sum::<usize>()
    }

    fn solve_part_2(&self) -> Self::Part2Output {
        let mut ranges = ThingRanges {
            x: vec![1, 4001],
            m: vec![1, 4001],
            a: vec![1, 4001],
            s: vec![1, 4001],
        };
        for flow in self.flows.values() {
            for stmt in &flow.0 {
                if let Statement::IfThen(cnd, _) = stmt {
                    match cnd {
                        Condition::LessThan(cat, val) => match cat {
                            Category::X => ranges.x.push(*val),
                            Category::M => ranges.m.push(*val),
                            Category::A => ranges.a.push(*val),
                            Category::S => ranges.s.push(*val),
                        },
                        Condition::GreaterThan(cat, val) => match cat {
                            Category::X => ranges.x.push(val + 1),
                            Category::M => ranges.m.push(val + 1),
                            Category::A => ranges.a.push(val + 1),
                            Category::S => ranges.s.push(val + 1),
                        },
                    }
                }
            }
        }
        ranges.x.sort();
        ranges.m.sort();
        ranges.a.sort();
        ranges.s.sort();
        let mut tot = 0;
        for (x1, x2) in ranges.x.iter().tuple_windows() {
            for (m1, m2) in ranges.m.iter().tuple_windows() {
                for (a1, a2) in ranges.a.iter().tuple_windows() {
                    for (s1, s2) in ranges.s.iter().tuple_windows() {
                        let thing = Thing {
                            x: *x1,
                            m: *m1,
                            a: *a1,
                            s: *s1,
                        };
                        if self.flows.get("in").unwrap().eval(&self.flows, &thing) {
                            tot += (x2 - x1) * (m2 - m1) * (a2 - a1) * (s2 - s1);
                        }
                    }
                }
            }
        }
        tot
    }

    fn solution(
        &self,
        input_type: crate::solution::InputType,
    ) -> (Option<Self::Part1Output>, Option<Self::Part2Output>) {
        match input_type {
            InputType::Examples => (Some(19114), Some(167409079868000)),
            InputType::Puzzles => (Some(401674), Some(134906204068564)),
        }
    }
}
