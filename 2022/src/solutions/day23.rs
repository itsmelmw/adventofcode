// https://adventofcode.com/2022/day/23

use std::collections::{HashSet, VecDeque};

use super::{InputParser, ProblemSolver};
type Point = (isize, isize);

enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    fn points_to_check(&self, loc: &Point) -> [Point; 3] {
        match self {
            Dir::North => [
                (loc.0 - 1, loc.1 - 1),
                (loc.0, loc.1 - 1),
                (loc.0 + 1, loc.1 - 1),
            ],
            Dir::East => [
                (loc.0 + 1, loc.1 - 1),
                (loc.0 + 1, loc.1),
                (loc.0 + 1, loc.1 + 1),
            ],
            Dir::South => [
                (loc.0 - 1, loc.1 + 1),
                (loc.0, loc.1 + 1),
                (loc.0 + 1, loc.1 + 1),
            ],
            Dir::West => [
                (loc.0 - 1, loc.1 - 1),
                (loc.0 - 1, loc.1),
                (loc.0 - 1, loc.1 + 1),
            ],
        }
    }
    fn step(&self, loc: &Point) -> Point {
        match self {
            Dir::North => (loc.0, loc.1 - 1),
            Dir::East => (loc.0 + 1, loc.1),
            Dir::South => (loc.0, loc.1 + 1),
            Dir::West => (loc.0 - 1, loc.1),
        }
    }
}

fn parse(input: &str) -> HashSet<Point> {
    let mut elves = HashSet::new();
    input.split('\n').enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c == '#' {
                elves.insert((x as isize, y as isize));
            }
        })
    });
    elves
}

fn surrounding_points(loc: &Point) -> [Point; 8] {
    [
        (loc.0 - 1, loc.1 - 1),
        (loc.0, loc.1 - 1),
        (loc.0 + 1, loc.1 - 1),
        (loc.0 - 1, loc.1),
        (loc.0 + 1, loc.1),
        (loc.0 - 1, loc.1 + 1),
        (loc.0, loc.1 + 1),
        (loc.0 + 1, loc.1 + 1),
    ]
}

fn do_iteration(curr_elves: &HashSet<Point>, dirs: &mut VecDeque<Dir>) -> HashSet<Point> {
    let mut next_elves = HashSet::new();
    'elves: for curr_elf in curr_elves.iter() {
        if surrounding_points(curr_elf)
            .iter()
            .any(|p| curr_elves.contains(p))
        {
            for dir in dirs.iter() {
                if dir
                    .points_to_check(curr_elf)
                    .iter()
                    .all(|p| !curr_elves.contains(p))
                {
                    let next_elf = &dir.step(curr_elf);
                    if next_elves.contains(next_elf) {
                        next_elves.remove(next_elf);
                        next_elves.insert(*curr_elf);
                        next_elves.insert(dir.step(&dir.step(curr_elf)));
                    } else {
                        next_elves.insert(*next_elf);
                    }
                    continue 'elves;
                }
            }
        }
        next_elves.insert(*curr_elf);
    }
    let dir = dirs.pop_front().unwrap();
    dirs.push_back(dir);
    next_elves
}

fn solve1(elves: &HashSet<Point>) -> String {
    let mut dirs = VecDeque::from([Dir::North, Dir::South, Dir::West, Dir::East]);
    let mut curr_elves = elves.clone();
    for _ in 0..10 {
        curr_elves = do_iteration(&curr_elves, &mut dirs);
    }
    let (mut min_x, mut min_y, mut max_x, mut max_y) =
        (isize::MAX, isize::MAX, isize::MIN, isize::MIN);

    for &(elf_x, elf_y) in curr_elves.iter() {
        if elf_x < min_x {
            min_x = elf_x;
        }
        if elf_x > max_x {
            max_x = elf_x
        }
        if elf_y < min_y {
            min_y = elf_y
        }
        if elf_y > max_y {
            max_y = elf_y
        }
    }
    ((max_x - min_x + 1) * (max_y - min_y + 1) - curr_elves.len() as isize).to_string()
}

fn solve2(elves: &HashSet<Point>) -> String {
    // Runs the first 10 iterations again, thus slightly inefficient.
    // Also could be tracked while moving instead of afterwards.
    let mut dirs = VecDeque::from([Dir::North, Dir::South, Dir::West, Dir::East]);
    let mut curr_elves = elves.clone();
    let mut new_elves;
    let mut iters = 0;
    loop {
        iters += 1;
        new_elves = do_iteration(&curr_elves, &mut dirs);
        if new_elves.iter().all(|p| curr_elves.contains(p)) {
            break;
        }
        curr_elves = new_elves;
    }
    iters.to_string()
}

pub struct Parser;

impl InputParser for Parser {
    type S = Solver;
    fn parse(input: &str) -> Solver {
        let elves = parse(input);
        Solver { elves }
    }
}

pub struct Solver {
    elves: HashSet<Point>,
}

impl ProblemSolver for Solver {
    fn solve_part_1(&self) -> String {
        solve1(&self.elves)
    }
    fn solve_part_2(&self) -> String {
        solve2(&self.elves)
    }
}
