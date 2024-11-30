// https://adventofcode.com/2023/day/25

use crate::solution::{InputType, Solution};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Clone)]
struct Edge<'i> {
    node1: &'i str,
    node2: &'i str,
}

pub struct Day25<'i> {
    nodes: HashSet<&'i str>,
    edges: Vec<Edge<'i>>,
    edge_map: HashMap<&'i str, Vec<usize>>,
}

impl<'i> Solution<'i> for Day25<'i> {
    type Part1Output = usize;
    type Part2Output = usize;

    fn title(&self) -> &str {
        "Snowverload"
    }

    fn parse(input: &'i str) -> Self {
        let mut edges = Vec::new();
        let mut edge_map = HashMap::new();
        let mut nodes = HashSet::new();
        input.split('\n').for_each(|line| {
            let (k, vs) = line.split_once(": ").unwrap();
            nodes.insert(k);
            for v in vs.split(' ') {
                nodes.insert(v);
                let edge = Edge { node1: k, node2: v };
                let idx = edges.len();
                edges.push(edge);
                edge_map.entry(k).or_insert(Vec::new()).push(idx);
                edge_map.entry(v).or_insert(Vec::new()).push(idx);
            }
        });
        Self {
            nodes,
            edges,
            edge_map,
        }
    }

    fn solve_part_1(&self) -> Self::Part1Output {
        let mut removed = Vec::new();
        for _ in 0..3 {
            let mut betweennesses = vec![0; self.edges.len()];
            for source in &self.nodes {
                self.update_betweennesses(source, &removed, &mut betweennesses)
            }
            let bridge_idx = betweennesses
                .iter()
                .enumerate()
                .max_by(|(_, v1), (_, v2)| v1.cmp(v2))
                .unwrap()
                .0;
            removed.push(bridge_idx);
        }
        let len = self.subgraph_len(self.edge_map.keys().next().unwrap(), &removed);
        println!("{} {}", len, self.nodes.len() - len);
        len * (self.nodes.len() - len)
    }

    fn solve_part_2(&self) -> Self::Part2Output {
        0
    }

    fn solution(
        &self,
        input_type: crate::solution::InputType,
    ) -> (Option<Self::Part1Output>, Option<Self::Part2Output>) {
        match input_type {
            InputType::Examples => (Some(54), Some(0)),
            InputType::Puzzles => (Some(506202), Some(0)),
        }
    }
}

impl Day25<'_> {
    fn update_betweennesses(
        &self,
        source: &str,
        removed_edges: &[usize],
        betweennesses: &mut [usize],
    ) {
        let mut queue = VecDeque::from([source]);
        let mut visited = HashSet::new();
        visited.insert(source);

        while let Some(node) = queue.pop_front() {
            let edge_idxs = &self.edge_map[node];
            for edge_idx in edge_idxs {
                if removed_edges.contains(edge_idx) {
                    continue;
                }
                let edge = &self.edges[*edge_idx];
                // let edge = &edges[*edge_idx];
                let node = if node == edge.node1 {
                    edge.node2
                } else {
                    edge.node1
                };
                if !visited.contains(node) {
                    queue.push_back(node);
                    visited.insert(node);
                    betweennesses[*edge_idx] += 1;
                }
            }
        }
    }
    fn subgraph_len(&self, source: &str, removed_edges: &[usize]) -> usize {
        let mut queue = VecDeque::from([source]);
        let mut visited = HashSet::new();
        visited.insert(source);

        while let Some(node) = queue.pop_front() {
            let edge_idxs = &self.edge_map[node];
            for edge_idx in edge_idxs {
                if removed_edges.contains(edge_idx) {
                    continue;
                }
                let edge = &self.edges[*edge_idx];
                let node = if node == edge.node1 {
                    edge.node2
                } else {
                    edge.node1
                };
                if !visited.contains(node) {
                    queue.push_back(node);
                    visited.insert(node);
                }
            }
        }
        visited.len()
    }
}
