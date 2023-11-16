// https://adventofcode.com/2022/day/18

use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

fn parse(input: &str) -> HashSet<(isize, isize, isize)> {
    return HashSet::from_iter(input.split('\n').map(|line| {
        line.split(',')
            .map(|num| num.parse::<isize>().unwrap())
            .collect_tuple::<(isize, isize, isize)>()
            .unwrap()
    }));
}

fn get_neighbors(loc: (isize, isize, isize)) -> [(isize, isize, isize); 6] {
    [
        (-1, 0, 0),
        (1, 0, 0),
        (0, -1, 0),
        (0, 1, 0),
        (0, 0, -1),
        (0, 0, 1),
    ]
    .map(|(dx, dy, dz)| (loc.0 + dx, loc.1 + dy, loc.2 + dz))
}

fn update_known(
    known: &mut HashMap<(isize, isize, isize), bool>,
    visited: &HashSet<(isize, isize, isize)>,
    inside: bool,
) -> bool {
    visited.iter().for_each(|loc| {
        known.insert(*loc, inside);
    });
    inside
}

// We define "inside" as: not being able to reach a border.
// Border is hardcoded as one of x, y or z being 0 or 21.
// We calculate this using BFS.
fn is_inside(
    known: &mut HashMap<(isize, isize, isize), bool>,
    blocks: &HashSet<(isize, isize, isize)>,
    loc: &(isize, isize, isize),
) -> bool {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(*loc);
    visited.insert(*loc);

    while let Some(next) = queue.pop_front() {
        if known.contains_key(loc) {
            return known[loc];
        }
        if next.0 <= 0 || next.0 >= 21 || next.1 <= 0 || next.1 >= 21 || next.2 <= 0 || next.2 >= 21
        {
            return update_known(known, &visited, false);
        }
        for neighbor in get_neighbors(next) {
            if !blocks.contains(&neighbor) && !visited.contains(&neighbor) {
                queue.push_back(neighbor);
                visited.insert(neighbor);
            }
        }
    }
    update_known(known, &visited, true)
}

fn solve1(parsed: &HashSet<(isize, isize, isize)>) -> String {
    parsed
        .iter()
        .map(|loc| {
            get_neighbors(*loc)
                .iter()
                .map(|loc| !parsed.contains(loc) as usize)
                .sum::<usize>()
        })
        .sum::<usize>()
        .to_string()
}

fn solve2(parsed: &HashSet<(isize, isize, isize)>) -> String {
    let mut known = HashMap::new();
    parsed
        .iter()
        .map(|loc| {
            get_neighbors(*loc)
                .iter()
                .map(|loc| (!parsed.contains(loc) && !is_inside(&mut known, parsed, loc)) as usize)
                .sum::<usize>()
        })
        .sum::<usize>()
        .to_string()
}

pub fn solve(input: &str) -> (String, String) {
    let parsed = parse(input);
    (solve1(&parsed), solve2(&parsed))
}
