// https://adventofcode.com/2023/day/8

use std::collections::HashMap;

use num::Integer;
use regex::Regex;

use aoc_utils::solutions::{InputDir, Part, Solution};

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

impl Solution for Day08 {
    fn title(&self) -> &str {
        "Haunted Wasteland"
    }
    fn parse(input: &str) -> Self {
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
    fn solve_part_1(&self) -> String {
        let mut curr = &"AAA".to_string();
        let mut step_num = 0;
        while curr != "ZZZ" {
            curr = self.do_step(curr, &self.steps[step_num % self.steps.len()]);
            step_num += 1;
        }
        step_num.to_string()
    }
    fn solve_part_2(&self) -> String {
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
            .to_string()
    }
    fn answer(&self, input: &InputDir, part: &Part) -> Option<&str> {
        match (input.name().as_str(), part) {
            ("Example", Part::One) => Some("2"),
            ("Example", Part::Two) => Some("6"),
            ("Puzzle", Part::One) => Some("18023"),
            ("Puzzle", Part::Two) => Some("14449445933179"),
            _ => unreachable!(),
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
