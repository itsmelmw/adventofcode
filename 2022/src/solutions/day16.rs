// https://adventofcode.com/2022/day/16

use std::collections::HashMap;

use regex::Regex;

struct Cave {
    valves: HashMap<String, Valve>,
}

impl Cave {
    pub fn new() -> Self {
        Self {
            valves: HashMap::new(),
        }
    }

    pub fn add_valve(&mut self, name: &str, rate: &str, tunnels: &str) {
        self.valves.insert(
            name.to_string(),
            Valve {
                name: name.to_string(),
                rate: rate.parse::<usize>().unwrap(),
                tunnels: tunnels
                    .split(", ")
                    .map(|name| name.to_string())
                    .collect::<Vec<String>>(),
            },
        );
    }
}

struct Valve {
    name: String,
    rate: usize,
    tunnels: Vec<String>,
}

fn parse(input: &str) -> Cave {
    let mut cave = Cave::new();
    let re =
        Regex::new(r"Valve (.*?) has flow rate=(.*?); tunnels? leads? to valves? (.+(?:, |$))+?")
            .unwrap();
    input.split("\n").for_each(|line| {
        let cap = re.captures(line).unwrap();
        cave.add_valve(&cap[1], &cap[2], &cap[3]);
    });
    return cave;
}

fn solve1(parsed: &Cave) -> String {
    return 0.to_string();
}

fn solve2(parsed: &Cave) -> String {
    return 0.to_string();
}

pub fn solve(input: &str) -> (String, String) {
    let parsed = parse(input);
    return (solve1(&parsed), solve2(&parsed));
}
