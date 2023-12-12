// https://adventofcode.com/2022/day/19

use std::ops::{Add, Mul, Sub};

use regex::Regex;

use aoc_utils::solutions::{InputDir, Part, Solution};

#[derive(Clone)]
struct State {
    time: usize,
    inv: Materials,
    robots: Materials,
}

impl State {
    fn start() -> Self {
        Self {
            time: 0,
            inv: Materials {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            robots: Materials {
                ore: 1,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
        }
    }
    fn build_robot(&self, cost: Materials, wait_time: usize, robot: Materials) -> State {
        State {
            time: self.time + wait_time,
            inv: self.inv + (self.robots * wait_time) - cost,
            robots: self.robots + robot,
        }
    }
}

#[derive(Clone, Default, Copy)]
struct Materials {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl Add for Materials {
    type Output = Materials;
    fn add(self, rhs: Self) -> Self::Output {
        Materials {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

impl Sub for Materials {
    type Output = Materials;
    fn sub(self, rhs: Self) -> Self::Output {
        Materials {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geode: self.geode - rhs.geode,
        }
    }
}

impl Mul<usize> for Materials {
    type Output = Materials;
    fn mul(self, rhs: usize) -> Self::Output {
        Materials {
            ore: self.ore * rhs,
            clay: self.clay * rhs,
            obsidian: self.obsidian * rhs,
            geode: self.geode * rhs,
        }
    }
}

struct Blueprint {
    ore_cost: Materials,
    clay_cost: Materials,
    obsidian_cost: Materials,
    geode_cost: Materials,
}

pub struct Day19 {
    blueprints: Vec<Blueprint>,
}

impl Solution for Day19 {
    fn title(&self) -> &str {
        "Not Enough Minerals"
    }
    fn parse(input: &str) -> Self {
        let re = Regex::new(r"Blueprint \d+: Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
        let blueprints = input
            .split('\n')
            .map(|line| {
                let cap = re.captures(line).unwrap();
                Blueprint {
                    ore_cost: Materials {
                        ore: cap[1].parse::<usize>().unwrap(),
                        ..Default::default()
                    },
                    clay_cost: Materials {
                        ore: cap[2].parse::<usize>().unwrap(),
                        ..Default::default()
                    },
                    obsidian_cost: Materials {
                        ore: cap[3].parse::<usize>().unwrap(),
                        clay: cap[4].parse::<usize>().unwrap(),
                        ..Default::default()
                    },
                    geode_cost: Materials {
                        ore: cap[5].parse::<usize>().unwrap(),
                        obsidian: cap[6].parse::<usize>().unwrap(),
                        ..Default::default()
                    },
                }
            })
            .collect::<Vec<Blueprint>>();
        Self { blueprints }
    }
    fn solve_part_1(&self) -> String {
        self.solve(&self.blueprints, 24)
            .iter()
            .enumerate()
            .map(|(i, v)| v * (i + 1))
            .sum::<usize>()
            .to_string()
    }
    fn solve_part_2(&self) -> String {
        self.solve(&self.blueprints[0..self.blueprints.len().min(3)], 32)
            .iter()
            .product::<usize>()
            .to_string()
    }
    fn answer(&self, input: &InputDir, part: &Part) -> Option<&str> {
        match (input.name().as_str(), part) {
            ("Example", Part::One) => Some("33"),
            ("Example", Part::Two) => Some("3472"),
            ("Puzzle", Part::One) => Some("1092"),
            ("Puzzle", Part::Two) => Some("3542"),
            _ => unreachable!(),
        }
    }
}

impl Day19 {
    fn solve(&self, blueprints: &[Blueprint], time_limit: usize) -> Vec<usize> {
        blueprints
            .iter()
            .map(|blueprint| {
                let maximums = Materials {
                    ore: blueprint
                        .ore_cost
                        .ore
                        .max(blueprint.clay_cost.ore)
                        .max(blueprint.obsidian_cost.ore)
                        .max(blueprint.geode_cost.ore),
                    clay: blueprint.obsidian_cost.clay,
                    obsidian: blueprint.geode_cost.obsidian,
                    ..Default::default()
                };
                Self::do_dfs(blueprint, &maximums, &State::start(), time_limit)
            })
            .collect::<Vec<usize>>()
    }
    fn do_dfs(
        blueprint: &Blueprint,
        maximums: &Materials,
        state: &State,
        time_limit: usize,
    ) -> usize {
        let mut score = state.inv.geode + (state.robots.geode * (time_limit - state.time));
        if state.robots.obsidian > 0 {
            let ore_wait = 1 + if state.inv.ore >= blueprint.geode_cost.ore {
                0
            } else {
                ((blueprint.geode_cost.ore - state.inv.ore) + state.robots.ore - 1)
                    / state.robots.ore
            };
            let obsidian_wait = 1 + if state.inv.obsidian >= blueprint.geode_cost.obsidian {
                0
            } else {
                ((blueprint.geode_cost.obsidian - state.inv.obsidian) + state.robots.obsidian - 1)
                    / state.robots.obsidian
            };
            let wait_time = ore_wait.max(obsidian_wait);
            if state.time + wait_time < time_limit {
                score = Self::do_dfs(
                    blueprint,
                    maximums,
                    &state.build_robot(
                        blueprint.geode_cost,
                        wait_time,
                        Materials {
                            geode: 1,
                            ..Default::default()
                        },
                    ),
                    time_limit,
                )
                .max(score);
            }
        }
        if state.robots.clay > 0 && state.robots.obsidian < maximums.obsidian {
            let ore_wait = 1 + if state.inv.ore >= blueprint.obsidian_cost.ore {
                0
            } else {
                ((blueprint.obsidian_cost.ore - state.inv.ore) + state.robots.ore - 1)
                    / state.robots.ore
            };
            let clay_wait = 1 + if state.inv.clay >= blueprint.obsidian_cost.clay {
                0
            } else {
                ((blueprint.obsidian_cost.clay - state.inv.clay) + state.robots.clay - 1)
                    / state.robots.clay
            };
            let wait_time = ore_wait.max(clay_wait);
            if state.time + wait_time < time_limit {
                score = Self::do_dfs(
                    blueprint,
                    maximums,
                    &state.build_robot(
                        blueprint.obsidian_cost,
                        wait_time,
                        Materials {
                            obsidian: 1,
                            ..Default::default()
                        },
                    ),
                    time_limit,
                )
                .max(score)
            }
        }
        if state.robots.clay < maximums.clay {
            let ore_wait = 1 + if state.inv.ore >= blueprint.clay_cost.ore {
                0
            } else {
                ((blueprint.clay_cost.ore - state.inv.ore) + state.robots.ore - 1)
                    / state.robots.ore
            };
            if state.time + ore_wait < time_limit {
                score = Self::do_dfs(
                    blueprint,
                    maximums,
                    &state.build_robot(
                        blueprint.clay_cost,
                        ore_wait,
                        Materials {
                            clay: 1,
                            ..Default::default()
                        },
                    ),
                    time_limit,
                )
                .max(score)
            }
        }
        if state.robots.ore < maximums.ore {
            let ore_wait = 1 + if state.inv.ore >= blueprint.ore_cost.ore {
                0
            } else {
                ((blueprint.ore_cost.ore - state.inv.ore) + state.robots.ore - 1) / state.robots.ore
            };
            if state.time + ore_wait < time_limit {
                score = Self::do_dfs(
                    blueprint,
                    maximums,
                    &state.build_robot(
                        blueprint.ore_cost,
                        ore_wait,
                        Materials {
                            ore: 1,
                            ..Default::default()
                        },
                    ),
                    time_limit,
                )
                .max(score)
            }
        }
        score
    }
}
