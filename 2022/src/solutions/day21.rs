// https://adventofcode.com/2022/day/21

use std::collections::HashMap;

use aoc_utils::solutions::{InputDir, Part, Solution};

type JobAssignments = HashMap<String, Job>;

enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

enum Job {
    Number(isize),
    Operator(String, Op, String),
    Human(isize),
}

impl Job {
    fn calculate(&self, assignments: &JobAssignments) -> isize {
        match self {
            Self::Number(num) | Self::Human(num) => *num,
            Self::Operator(lhs, op, rhs) => {
                let lhs = assignments.get(lhs).unwrap().calculate(assignments);
                let rhs = assignments.get(rhs).unwrap().calculate(assignments);
                match op {
                    Op::Add => lhs + rhs,
                    Op::Sub => lhs - rhs,
                    Op::Mul => lhs * rhs,
                    Op::Div => lhs / rhs,
                }
            }
        }
    }
    fn calculate_human(&self, assignments: &JobAssignments) -> isize {
        if let Self::Operator(lhs, _, rhs) = self {
            let lhs = assignments.get(lhs).unwrap();
            let rhs = assignments.get(rhs).unwrap();
            let (human_branch, monkey_branch) = if lhs.contains_human(assignments) {
                (lhs, rhs)
            } else {
                (rhs, lhs)
            };
            let base_value = monkey_branch.calculate(assignments);
            return human_branch.reverse_calculate(assignments, base_value);
        }
        panic!();
    }
    fn contains_human(&self, assignments: &JobAssignments) -> bool {
        // This could all be calculated at once, instead of recalculating for every branch.
        match self {
            Self::Number(_) => false,
            Self::Human(_) => true,
            Self::Operator(lhs, _, rhs) => {
                assignments.get(lhs).unwrap().contains_human(assignments)
                    || assignments.get(rhs).unwrap().contains_human(assignments)
            }
        }
    }
    fn reverse_calculate(&self, assignments: &JobAssignments, cur_value: isize) -> isize {
        match self {
            Self::Operator(lhs, op, rhs) => {
                let lhs = assignments.get(lhs).unwrap();
                let rhs = assignments.get(rhs).unwrap();
                if lhs.contains_human(assignments) {
                    let other_value = rhs.calculate(assignments);
                    let new_value = match op {
                        Op::Add => cur_value - other_value,
                        Op::Sub => cur_value + other_value,
                        Op::Mul => cur_value / other_value,
                        Op::Div => cur_value * other_value,
                    };
                    lhs.reverse_calculate(assignments, new_value)
                } else {
                    let other_value = lhs.calculate(assignments);
                    let new_value = match op {
                        Op::Add => cur_value - other_value,
                        Op::Sub => other_value - cur_value,
                        Op::Mul => cur_value / other_value,
                        Op::Div => other_value / cur_value,
                    };
                    rhs.reverse_calculate(assignments, new_value)
                }
            }
            Self::Human(_) => cur_value,
            Self::Number(_) => panic!(),
        }
    }
}

pub struct Day21 {
    assignments: JobAssignments,
}

impl Solution for Day21 {
    fn title(&self) -> &str {
        "Monkey Math"
    }
    fn parse(input: &str) -> Self {
        let assignments = input
            .split('\n')
            .map(|line| {
                let (monkey, job) = line.split_once(": ").unwrap();
                let job = match job.split(' ').collect::<Vec<&str>>()[..] {
                    [num] if monkey == "humn" => Job::Human(num.parse().unwrap()),
                    [num] => Job::Number(num.parse().unwrap()),
                    [lhs, "+", rhs] => Job::Operator(lhs.to_string(), Op::Add, rhs.to_string()),
                    [lhs, "-", rhs] => Job::Operator(lhs.to_string(), Op::Sub, rhs.to_string()),
                    [lhs, "*", rhs] => Job::Operator(lhs.to_string(), Op::Mul, rhs.to_string()),
                    [lhs, "/", rhs] => Job::Operator(lhs.to_string(), Op::Div, rhs.to_string()),
                    _ => unreachable!(),
                };
                (monkey.to_string(), job)
            })
            .collect::<JobAssignments>();
        Self { assignments }
    }
    fn solve_part_1(&self) -> String {
        self.assignments
            .get("root")
            .unwrap()
            .calculate(&self.assignments)
            .to_string()
    }
    fn solve_part_2(&self) -> String {
        self.assignments
            .get("root")
            .unwrap()
            .calculate_human(&self.assignments)
            .to_string()
    }
    fn answer(&self, input: &InputDir, part: &Part) -> Option<&str> {
        match (input.name().as_str(), part) {
            ("Example", Part::One) => Some("152"),
            ("Example", Part::Two) => Some("301"),
            ("Puzzle", Part::One) => Some("41857219607906"),
            ("Puzzle", Part::Two) => Some("3916936880448"),
            _ => unreachable!(),
        }
    }
}
