// https://adventofcode.com/2022/day/16

use std::collections::{HashMap, HashSet, VecDeque};

use regex::Regex;

struct Cave {
    valves: HashMap<String, Valve>,
    nonzero: usize,
}

impl Cave {
    pub fn new() -> Self {
        Self {
            valves: HashMap::new(),
            nonzero: 0,
        }
    }

    pub fn add_valve(&mut self, name: &str, rate: &str, tunnels: &str) {
        let rate = rate.parse::<usize>().unwrap();
        self.valves.insert(
            name.to_string(),
            Valve {
                name: name.to_string(),
                rate,
                tunnels: tunnels
                    .split(", ")
                    .map(|name| name.to_string())
                    .collect::<Vec<String>>(),
            },
        );
        if rate > 0 {
            self.nonzero += 1;
        }
    }
}

struct Valve {
    name: String,
    rate: usize,
    tunnels: Vec<String>,
}

struct Position {
    score: usize,
    valve: String,
    mins_left: usize,
    opened: HashSet<String>,
}

impl Position {
    fn new(score: usize, valve: String, mins_left: usize, opened: HashSet<String>) -> Self {
        Self {
            score,
            valve: valve.to_string(),
            mins_left,
            opened,
        }
    }
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

fn solve1(cave: &Cave) -> String {
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();
    queue.push_back(Position::new(0, "AA".to_string(), 30, HashSet::new()));
    let mut best_score = 0;

    while !queue.is_empty() {
        let curr = queue.pop_front().unwrap();
        let valve = cave.valves.get(&curr.valve).unwrap();
        let mins_left = curr.mins_left - 1;

        if let Some(v) = visited.get(&valve.name) {
            if *v >= curr.score {
                continue;
            }
        }

        if mins_left == 0 || cave.nonzero == curr.opened.len() {
            if curr.score > best_score {
                best_score = curr.score
            }
            continue;
        }
        visited.insert(valve.name.to_string(), curr.score);

        if !curr.opened.contains(&valve.name) && valve.rate != 0 {
            let open_score = cave.valves.get(&valve.name).unwrap().rate * mins_left;
            let mut new_opened = curr.opened.clone();
            new_opened.insert(valve.name.to_string());
            queue.push_back(Position::new(
                curr.score + open_score,
                curr.valve,
                mins_left,
                new_opened,
            ));
        }
        for new_valve in &valve.tunnels {
            queue.push_back(Position::new(
                curr.score,
                new_valve.to_string(),
                mins_left,
                curr.opened.clone(),
            ));
        }
    }

    return best_score.to_string();
}

struct DoublePosition {
    score: usize,
    human_valve: String,
    elephant_valve: String,
    mins_left: usize,
    opened: HashSet<String>,
}

impl DoublePosition {
    fn new(
        score: usize,
        human_valve: String,
        elephant_valve: String,
        mins_left: usize,
        opened: HashSet<String>,
    ) -> Self {
        Self {
            score,
            human_valve: human_valve.to_string(),
            elephant_valve: elephant_valve.to_string(),
            mins_left,
            opened,
        }
    }
}

fn solve2(cave: &Cave) -> String {
    // This can definitely be done better, but I'll stick with the naive solution here.
    // This still only takes a bit more than one second.
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();
    queue.push_back(DoublePosition::new(
        0,
        "AA".to_string(),
        "AA".to_string(),
        26,
        HashSet::new(),
    ));
    let mut best_score = 0;

    while !queue.is_empty() {
        let curr = queue.pop_front().unwrap();
        let human_valve = cave.valves.get(&curr.human_valve).unwrap();
        let elephant_valve = cave.valves.get(&curr.elephant_valve).unwrap();
        let mins_left = curr.mins_left - 1;

        if let Some(v) = visited.get(&(
            human_valve.name.to_string(),
            elephant_valve.name.to_string(),
        )) {
            if *v >= curr.score {
                continue;
            }
        }

        if mins_left == 0 || cave.nonzero == curr.opened.len() {
            if curr.score > best_score {
                best_score = curr.score
            }
            continue;
        }
        visited.insert(
            (
                human_valve.name.to_string(),
                elephant_valve.name.to_string(),
            ),
            curr.score,
        );

        let mut human_positions = vec![];

        if !curr.opened.contains(&human_valve.name) && human_valve.rate != 0 {
            let open_score = cave.valves.get(&human_valve.name).unwrap().rate * mins_left;
            let mut new_opened = curr.opened.clone();
            new_opened.insert(human_valve.name.to_string());
            human_positions.push(DoublePosition::new(
                curr.score + open_score,
                curr.human_valve.to_string(),
                curr.elephant_valve.to_string(),
                mins_left,
                new_opened,
            ));
        }
        for new_valve in &human_valve.tunnels {
            human_positions.push(DoublePosition::new(
                curr.score,
                new_valve.to_string(),
                curr.elephant_valve.to_string(),
                mins_left,
                curr.opened.clone(),
            ));
        }

        for curr in human_positions {
            if !curr.opened.contains(&elephant_valve.name) && elephant_valve.rate != 0 {
                let open_score = cave.valves.get(&elephant_valve.name).unwrap().rate * mins_left;
                let mut new_opened = curr.opened.clone();
                new_opened.insert(elephant_valve.name.to_string());
                queue.push_back(DoublePosition::new(
                    curr.score + open_score,
                    curr.human_valve.to_string(),
                    curr.elephant_valve.to_string(),
                    mins_left,
                    new_opened,
                ));
            }
            for new_valve in &elephant_valve.tunnels {
                queue.push_back(DoublePosition::new(
                    curr.score,
                    curr.human_valve.to_string(),
                    new_valve.to_string(),
                    mins_left,
                    curr.opened.clone(),
                ));
            }
        }
    }

    return best_score.to_string();
}

pub fn solve(input: &str) -> (String, String) {
    let parsed = parse(input);
    return (solve1(&parsed), solve2(&parsed));
}
