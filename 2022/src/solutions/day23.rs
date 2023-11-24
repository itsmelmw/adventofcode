// https://adventofcode.com/2022/day/23

use crate::solutions::{InputParser, ProblemSolver};
use crate::utils::{Dir, IPoint};
use std::collections::{HashSet, VecDeque};

fn parse(input: &str) -> HashSet<IPoint> {
    let mut elves = HashSet::new();
    input.split('\n').enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c == '#' {
                elves.insert(IPoint::new(x as isize, y as isize));
            }
        })
    });
    elves
}

fn do_iteration(curr_elves: &HashSet<IPoint>, dirs: &mut VecDeque<Dir>) -> HashSet<IPoint> {
    let mut next_elves = HashSet::new();
    'elves: for curr_elf in curr_elves.iter() {
        if !curr_elf
            .neighbors_8()
            .iter()
            .any(|p| curr_elves.contains(p))
        {
            next_elves.insert(*curr_elf);
            continue;
        }

        for dir in dirs.iter() {
            if curr_elf
                .dir_wide(dir)
                .iter()
                .all(|p| !curr_elves.contains(p))
            {
                let next_elf = &curr_elf.dir(dir);
                if next_elves.contains(next_elf) {
                    next_elves.remove(next_elf);
                    next_elves.insert(*curr_elf);
                    next_elves.insert(curr_elf.dir(dir).dir(dir));
                } else {
                    next_elves.insert(*next_elf);
                }
                continue 'elves;
            }
        }

        next_elves.insert(*curr_elf);
    }
    let dir = dirs.pop_front().unwrap();
    dirs.push_back(dir);
    next_elves
}

fn solve1(elves: &HashSet<IPoint>) -> String {
    let mut dirs = VecDeque::from([Dir::Up, Dir::Down, Dir::Left, Dir::Right]);
    let mut curr_elves = elves.clone();
    for _ in 0..10 {
        curr_elves = do_iteration(&curr_elves, &mut dirs);
    }
    let (mut min_x, mut min_y, mut max_x, mut max_y) =
        (isize::MAX, isize::MAX, isize::MIN, isize::MIN);
    for elf in curr_elves.iter() {
        if elf.x < min_x {
            min_x = elf.x;
        }
        if elf.x > max_x {
            max_x = elf.x;
        }
        if elf.y < min_y {
            min_y = elf.y;
        }
        if elf.y > max_y {
            max_y = elf.y;
        }
    }
    ((max_x - min_x + 1) * (max_y - min_y + 1) - curr_elves.len() as isize).to_string()
}

fn solve2(elves: &HashSet<IPoint>) -> String {
    // Runs the first 10 iterations again, thus slightly inefficient.
    // Also could be tracked while moving instead of afterwards.
    let mut dirs = VecDeque::from([Dir::Up, Dir::Down, Dir::Left, Dir::Right]);
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
    elves: HashSet<IPoint>,
}

impl ProblemSolver for Solver {
    fn solve_part_1(&self) -> String {
        solve1(&self.elves)
    }
    fn solve_part_2(&self) -> String {
        solve2(&self.elves)
    }
}
