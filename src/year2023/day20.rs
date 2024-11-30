// https://adventofcode.com/2023/day/20

use crate::solution::{InputType, Solution};
use num::Integer;
use std::collections::{HashMap, VecDeque};

#[derive(Clone)]
enum Module {
    FlipFlop(bool),
    Conjunction(HashMap<String, Pulse>),
    None,
}

impl Module {
    fn handle_pulse(&mut self, src: &String, pulse: &Pulse) -> Option<Pulse> {
        match self {
            Self::FlipFlop(is_on) => match pulse {
                Pulse::High => None,
                Pulse::Low => {
                    *is_on = !*is_on;
                    if *is_on {
                        Some(Pulse::High)
                    } else {
                        Some(Pulse::Low)
                    }
                }
            },
            Self::Conjunction(map) => {
                map.insert(src.to_string(), *pulse);
                if map.values().all(|&v| v == Pulse::High) {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }
            Self::None => Some(*pulse),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Pulse {
    Low,
    High,
}

type Configuration = HashMap<String, (Module, Vec<String>)>;

pub struct Day20 {
    config: Configuration,
}

impl<'i> Solution<'i> for Day20 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn title(&self) -> &str {
        "Pulse Propagation"
    }

    fn parse(input: &'i str) -> Self {
        let mut config = input
            .split('\n')
            .map(|line| {
                let (module, cables) = line.split_once(" -> ").unwrap();
                let (module_type, module_label) = match module.split_at(1) {
                    ("%", label) => (Module::FlipFlop(false), label.to_string()),
                    ("&", label) => (Module::Conjunction(HashMap::new()), label.to_string()),
                    _ => (Module::None, module.to_string()),
                };
                let cables = cables
                    .split(", ")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                (module_label, (module_type, cables))
            })
            .collect::<Configuration>();

        // Fill the maps of all conjunctions with its sources.
        let src_dsts = config
            .iter()
            .map(|(src, (_, dsts))| (src.clone(), dsts.clone()))
            .collect::<Vec<(String, Vec<String>)>>();

        for (src, dsts) in src_dsts {
            for dst in dsts {
                if let Some((Module::Conjunction(map), _)) = config.get_mut(&dst) {
                    map.insert(src.to_string(), Pulse::Low);
                }
            }
        }
        Self { config }
    }

    fn solve_part_1(&self) -> Self::Part1Output {
        let iters = 1000;
        let (mut low, mut high) = (iters, 0);
        let mut config = self.config.clone();
        for _ in 0..iters {
            self.press_button(&mut config, |_, _, pulse| match pulse {
                Pulse::Low => low += 1,
                Pulse::High => high += 1,
            });
        }
        low * high
    }

    fn solve_part_2(&self) -> Self::Part2Output {
        // The way the input is structured for the puzzle, `rx` receives a low
        // pulse from a conjunction (`zr`) that relies on 4 more conjunctions pulsing
        // high at the same time. Thus, the button presses it takes to power `rx` is
        // the LCM of the button presses it takes for each of these conjunctions to
        // "cycle".
        // This solution relies on the input to be structured like this.
        let mut config = self.config.clone();

        // First, get the module that pulses to `rx`.
        let rx_pulser = config
            .iter()
            .find_map(|(src, (_, dsts))| {
                if dsts.contains(&"rx".to_string()) || dsts.contains(&"output".to_string()) {
                    Some(src.clone())
                } else {
                    None
                }
            })
            .unwrap();

        // Then, find the modules that pulse to this module.
        let dep_modules = config
            .iter()
            .filter_map(|(src, (_, dsts))| {
                if dsts.contains(&rx_pulser) {
                    Some(src.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<String>>();

        // Finally, for each of these modules, count the number of
        // button presses it takes between two times it pulses "low".
        dep_modules
            .iter()
            .map(|module| {
                let mut press_count = 0;
                let mut high_presses = [0; 2];
                let mut pulses = 0;
                while pulses < 2 {
                    press_count += 1;
                    self.press_button(&mut config, |src, dst, &pulse| {
                        if src == module && *dst == rx_pulser && pulse == Pulse::High {
                            high_presses[pulses] = press_count;
                            pulses += 1;
                        }
                    });
                }
                high_presses[1] - high_presses[0]
            })
            .fold(1usize, |x, y| x.lcm(&y))
    }

    fn solution(
        &self,
        input_type: crate::solution::InputType,
    ) -> (Option<Self::Part1Output>, Option<Self::Part2Output>) {
        match input_type {
            InputType::Examples => (Some(11687500), Some(4)),
            InputType::Puzzles => (Some(670984704), Some(262775362119547)),
        }
    }
}

impl Day20 {
    fn press_button<F>(&self, config: &mut Configuration, mut pulse_fn: F)
    where
        F: FnMut(&String, &String, &Pulse),
    {
        let mut queue = VecDeque::from(vec![(
            "button".to_string(),
            "broadcaster".to_string(),
            Pulse::Low,
        )]);
        while let Some((src, dst, pulse)) = queue.pop_front() {
            if let Some((module, dsts)) = config.get_mut(&dst) {
                if let Some(new_pulse) = module.handle_pulse(&src, &pulse) {
                    for new_dst in dsts.clone() {
                        let src = dst.clone();
                        pulse_fn(&src, &new_dst, &new_pulse);
                        queue.push_back((src, new_dst, new_pulse));
                    }
                }
            }
        }
    }
}
