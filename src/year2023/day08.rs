// https://adventofcode.com/2023/day/8

use crate::solution::{InputType, Solution};
use num::Integer;
use regex::Regex;
use std::collections::HashMap;

enum Step {
    Left,
    Right,
}

type Tree = HashMap<String, Node>;

struct Node {
    left: String,
    right: String,
}

pub struct Day08 {
    steps: Vec<Step>,
    tree: Tree,
    starts: Vec<String>,
}

impl<'i> Solution<'i> for Day08 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn title(&self) -> &str {
        "Haunted Wasteland"
    }

    fn parse(input: &'i str) -> Self {
        let tree_re = Regex::new(r"([A-Z0-9]{3}) = \(([A-Z0-9]{3}), ([A-Z0-9]{3})\)").unwrap();
        let (steps, tree, starts) = input
            .split_once("\n\n")
            .map(|(steps, tree)| {
                let steps = steps
                    .chars()
                    .map(|c| match c {
                        'L' => Step::Left,
                        'R' => Step::Right,
                        _ => unreachable!(),
                    })
                    .collect::<Vec<Step>>();
                let mut starts = Vec::new();
                let tree = tree
                    .split('\n')
                    .map(|line| {
                        let cap = tree_re.captures(line).unwrap();
                        let node = Node {
                            left: cap[2].to_string(),
                            right: cap[3].to_string(),
                        };
                        if cap[1].ends_with('A') {
                            starts.push(cap[1].to_string())
                        }
                        (cap[1].to_string(), node)
                    })
                    .collect::<Tree>();
                (steps, tree, starts)
            })
            .unwrap();
        Self {
            steps,
            tree,
            starts,
        }
    }

    fn solve_part_1(&self) -> Self::Part1Output {
        let mut curr = &"AAA".to_string();
        let mut step_num = 0;
        while curr != "ZZZ" {
            curr = self.do_step(curr, &self.steps[step_num % self.steps.len()]);
            step_num += 1;
        }
        step_num
    }

    fn solve_part_2(&self) -> Self::Part2Output {
        self.starts
            .iter()
            .map(|mut curr| {
                let mut step_num = 0;
                while !curr.ends_with('Z') {
                    curr = self.do_step(curr, &self.steps[step_num % self.steps.len()]);
                    step_num += 1;
                }
                step_num
            })
            .fold(1usize, |x, y| x.lcm(&y))
    }

    fn solution(
        &self,
        input_type: crate::solution::InputType,
    ) -> (Option<Self::Part1Output>, Option<Self::Part2Output>) {
        match input_type {
            InputType::Examples => (Some(2), Some(6)),
            InputType::Puzzles => (Some(18023), Some(14449445933179)),
        }
    }
}

impl Day08 {
    fn do_step(&self, curr: &String, step: &Step) -> &String {
        let node = &self.tree[curr];
        match step {
            Step::Left => &node.left,
            Step::Right => &node.right,
        }
    }
}
