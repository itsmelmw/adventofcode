// https://adventofcode.com/2022/day/23

use crate::grids::{Dir, IPoint};
use crate::solution::{InputType, Day};
use std::collections::{HashSet, VecDeque};

pub struct Day23 {
    elves: HashSet<IPoint>,
}

impl<'i> Day<'i> for Day23 {
    type Part1Output = isize;
    type Part2Output = isize;

    fn title(&self) -> &str {
        "Unstable Diffusion"
    }

    fn parse(input: &'i str) -> Self {
        let mut elves = HashSet::new();
        input.split('\n').enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                if c == '#' {
                    elves.insert(IPoint::new(x as isize, y as isize));
                }
            })
        });
        Self { elves }
    }

    fn solve_part_1(&self) -> Self::Part1Output {
        let mut dirs = VecDeque::from([Dir::Up, Dir::Down, Dir::Left, Dir::Right]);
        let mut curr_elves = self.elves.clone();
        for _ in 0..10 {
            curr_elves = self.do_iteration(&curr_elves, &mut dirs);
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
        (max_x - min_x + 1) * (max_y - min_y + 1) - curr_elves.len() as isize
    }

    fn solve_part_2(&self) -> Self::Part2Output {
        // Runs the first 10 iterations again, thus slightly inefficient.
        // Also could be tracked while moving instead of afterwards.
        let mut dirs = VecDeque::from([Dir::Up, Dir::Down, Dir::Left, Dir::Right]);
        let mut curr_elves = self.elves.clone();
        let mut new_elves;
        let mut iters = 0;
        loop {
            iters += 1;
            new_elves = self.do_iteration(&curr_elves, &mut dirs);
            if new_elves.iter().all(|p| curr_elves.contains(p)) {
                break;
            }
            curr_elves = new_elves;
        }
        iters
    }

    fn solution(
        &self,
        input_type: crate::solution::InputType,
    ) -> (Option<Self::Part1Output>, Option<Self::Part2Output>) {
        match input_type {
            InputType::Examples => (Some(110), Some(20)),
            InputType::Puzzles => (Some(4158), Some(1014)),
        }
    }
}

impl Day23 {
    fn do_iteration(
        &self,
        curr_elves: &HashSet<IPoint>,
        dirs: &mut VecDeque<Dir>,
    ) -> HashSet<IPoint> {
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
                        next_elves.insert(curr_elf.dir_steps(dir, 2));
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
}
