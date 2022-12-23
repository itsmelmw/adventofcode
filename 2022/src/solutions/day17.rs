// https://adventofcode.com/2022/day/17

use std::collections::{HashMap, HashSet};

#[derive(Clone)]
enum Jet {
    Left,
    Right,
}

struct JetPattern {
    jets: Vec<Jet>,
    pub current: usize,
}

impl JetPattern {
    pub fn new(jets: Vec<Jet>) -> Self {
        Self {
            jets: jets,
            current: 0,
        }
    }

    pub fn next(&mut self) -> &Jet {
        let jet = &self.jets[self.current];
        self.current += 1;
        if self.current >= self.jets.len() {
            self.current = 0;
        }
        return jet;
    }
}

struct Rock {
    height: usize,
    pieces: Vec<(usize, usize)>,
}

impl Rock {
    pub fn new(pieces: Vec<(usize, usize)>) -> Self {
        Self {
            height: *pieces.iter().map(|(_, y)| y).max().unwrap() + 1,
            pieces: pieces,
        }
    }
}

struct RockPattern {
    rocks: Vec<Rock>,
    pub current: usize,
}

impl RockPattern {
    pub fn new(rocks: Vec<Vec<(usize, usize)>>) -> Self {
        Self {
            rocks: rocks
                .iter()
                .map(|rock| Rock::new(rock.to_vec()))
                .collect::<Vec<Rock>>(),
            current: 0,
        }
    }

    pub fn next(&mut self) -> &Rock {
        let rock = &self.rocks[self.current];
        self.current += 1;
        if self.current >= self.rocks.len() {
            self.current = 0;
        }
        return rock;
    }
}

struct RockMap {
    map: HashSet<(usize, usize)>,
    height: usize,
    rel_cols: [usize; 7],
}

impl RockMap {
    pub fn new(width: usize) -> Self {
        Self {
            map: HashSet::new(),
            height: 0,
            rel_cols: [0; 7],
        }
    }

    pub fn touches_side(&self, rock: &Rock, loc: (usize, usize), jet: &Jet) -> bool {
        let dx = if let Jet::Left = jet { -1 } else { 1 };
        return rock.pieces.iter().any(|(rx, ry)| {
            let newx = (loc.0 as isize + *rx as isize + dx) as usize;
            newx == 0 || newx > 7 || self.map.contains(&(newx, loc.1 - ry))
        });
    }

    pub fn touches_bottom(&self, rock: &Rock, loc: (usize, usize)) -> bool {
        return rock.pieces.iter().any(|(rx, ry)| {
            let newy = loc.1 - ry - 1;
            self.map.contains(&(loc.0 + rx, newy)) || newy == 0
        });
    }

    pub fn insert(&mut self, rock: &Rock, loc: (usize, usize)) {
        // Update total height and relative column heights
        if loc.1 > self.height {
            self.rel_cols
                .iter_mut()
                .for_each(|col| *col += loc.1 - self.height);
            self.height = loc.1;
        }

        rock.pieces.iter().for_each(|(rx, ry)| {
            let (x, y) = (loc.0 + rx, loc.1 - ry);
            self.map.insert((x, y));
            // Update relative column heights
            if self.height - y < self.rel_cols[x - 1] {
                self.rel_cols[x - 1] = self.height - y;
            }
        });
    }
}

fn parse(input: &str) -> Vec<Jet> {
    return input
        .bytes()
        .map(|b| match b {
            b'<' => Jet::Left,
            b'>' => Jet::Right,
            _ => unreachable!(),
        })
        .collect::<Vec<Jet>>();
}

fn drop_rocks(jets: &Vec<Jet>, num: usize) -> usize {
    let mut map = RockMap::new(7);
    let mut jet_pattern = JetPattern::new(jets.to_vec());
    let mut rock_pattern = RockPattern::new(vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        vec![(2, 0), (2, 1), (0, 2), (1, 2), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (1, 0), (0, 1), (1, 1)],
    ]);
    let mut column_pattern = HashMap::new();

    let mut cut_height = 0;
    let mut dropped = 0;
    while dropped < num {
        let rock = rock_pattern.next();
        let mut loc = (3, map.height + 3 + rock.height);

        // Simulate dropping one rock
        loop {
            let jet = jet_pattern.next();
            if !map.touches_side(rock, loc, jet) {
                loc.0 = if let Jet::Left = jet {
                    loc.0 - 1
                } else {
                    loc.0 + 1
                };
            }
            if map.touches_bottom(rock, loc) {
                break;
            } else {
                loc.1 -= 1;
            }
        }

        // Update the map
        map.insert(rock, loc);

        // Check if we found a pattern, by checking if we had this combination of
        // relative column height, current rock, and current jet before.
        // If so, check how often the pattern fits in the space and apply it that many times.
        if cut_height == 0 {
            if let Some((prev_height, prev_dropped)) = column_pattern.insert(
                (map.rel_cols, rock_pattern.current, jet_pattern.current),
                (map.height, dropped),
            ) {
                let pattern_height = map.height - prev_height;
                let pattern_drop = dropped - prev_dropped;
                let pattern_fits = (num - dropped - 1) / pattern_drop;

                cut_height = pattern_fits * pattern_height;
                dropped += pattern_fits * pattern_drop;
            }
        }
        dropped += 1;
    }

    return map.height + cut_height;
}

fn solve1(parsed: &Vec<Jet>) -> String {
    return drop_rocks(parsed, 2022).to_string();
}

fn solve2(parsed: &Vec<Jet>) -> String {
    return drop_rocks(parsed, 1_000_000_000_000).to_string();
}

pub fn solve(input: &str) -> (String, String) {
    let parsed = parse(input);
    return (solve1(&parsed), solve2(&parsed));
}
