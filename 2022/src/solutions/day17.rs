// https://adventofcode.com/2022/day/17

use std::collections::HashSet;

#[derive(Clone)]
enum Jet {
    Left,
    Right,
}

struct JetPattern {
    jets: Vec<Jet>,
    current: usize,
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
    width: usize,
    height: usize,
    pieces: Vec<(usize, usize)>,
}

impl Rock {
    pub fn new(pieces: Vec<(usize, usize)>) -> Self {
        Self {
            width: *pieces.iter().map(|(x, _)| x).max().unwrap() + 1,
            height: *pieces.iter().map(|(_, y)| y).max().unwrap() + 1,
            pieces: pieces,
        }
    }
}

struct RockPattern {
    rocks: Vec<Rock>,
    current: usize,
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
    width: usize,
    height: usize,
}

impl RockMap {
    pub fn new(width: usize) -> Self {
        Self {
            map: HashSet::new(),
            width: width,
            height: 0,
        }
    }

    pub fn touches_side(&self, rock: &Rock, loc: (usize, usize), jet: &Jet) -> bool {
        let dx = if let Jet::Left = jet { -1 } else { 1 };
        return rock.pieces.iter().any(|(rx, ry)| {
            let newx = (loc.0 as isize + *rx as isize + dx) as usize;
            newx == 0 || newx > self.width || self.map.contains(&(newx, loc.1 - ry))
        });
    }

    pub fn touches_bottom(&self, rock: &Rock, loc: (usize, usize)) -> bool {
        return rock.pieces.iter().any(|(rx, ry)| {
            let newy = loc.1 - ry - 1;
            self.map.contains(&(loc.0 + rx, newy)) || newy == 0
        });
    }

    pub fn insert(&mut self, rock: &Rock, loc: (usize, usize)) {
        if loc.1 > self.height {
            self.height = loc.1;
        }
        rock.pieces.iter().for_each(|(rx, ry)| {
            self.map.insert((loc.0 + rx, loc.1 - ry));
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

fn solve1(parsed: &Vec<Jet>) -> String {
    let mut map = RockMap::new(7);
    let mut jet_pattern = JetPattern::new(parsed.to_vec());
    let mut rock_pattern = RockPattern::new(vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        vec![(2, 0), (2, 1), (0, 2), (1, 2), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (1, 0), (0, 1), (1, 1)],
    ]);

    for _ in 0..2022 {
        let rock = rock_pattern.next();
        let mut loc = (3, map.height + 3 + rock.height);

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
        map.insert(rock, loc);
    }

    return map.height.to_string();
}

fn solve2(parsed: &Vec<Jet>) -> String {
    let mut map = RockMap::new(7);
    let mut jet_pattern = JetPattern::new(parsed.to_vec());
    let mut rock_pattern = RockPattern::new(vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        vec![(2, 0), (2, 1), (0, 2), (1, 2), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (1, 0), (0, 1), (1, 1)],
    ]);

    for _ in 0..1_000_000_000_000usize {
        let rock = rock_pattern.next();
        let mut loc = (3, map.height + 3 + rock.height);

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
        map.insert(rock, loc);
    }

    return map.height.to_string();
}

pub fn solve(input: &str) -> (String, String) {
    let parsed = parse(input);
    return (solve1(&parsed), solve2(&parsed));
}
