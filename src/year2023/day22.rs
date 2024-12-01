// https://adventofcode.com/2023/day/22

use crate::{
    grids::{Grid, UPoint},
    solution::{Day, InputType},
};
use std::ops::Deref;

type Slab = Vec<Point3d>;
type Point3d = (usize, usize, usize);

pub struct Day22 {
    slabs: Vec<Slab>,
}

impl<'i> Day<'i> for Day22 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn title(&self) -> &str {
        "Sand Slabs"
    }

    fn parse(input: &'i str) -> Self {
        let mut slabs = input
            .split('\n')
            .map(|line| {
                let (start, end) = line.split_once('~').unwrap();
                let start = start
                    .split(',')
                    .map(|i| i.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                let end = end
                    .split(',')
                    .map(|i| i.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();

                if let Some(diff_idx) = start.iter().zip(end.iter()).position(|ax| ax.0 != ax.1) {
                    let mut curr = start;
                    let mut slab = Vec::new();
                    while curr[diff_idx] <= end[diff_idx] {
                        slab.push((curr[0], curr[1], curr[2]));
                        curr[diff_idx] += 1;
                    }
                    slab
                } else {
                    vec![(start[0], start[1], start[2])]
                }
            })
            .collect::<Vec<Slab>>();

        // Drop all the slabs already, since it's needed for both parts.
        // It could be possible to instead structure the slabs as a tree
        // based on which slabs support which, which could be nicer.
        // For now, I implemented it the straightforward way.
        let mut height_map = Grid::from_vec(vec![0; 100], 10);
        slabs.sort_by(|a, b| a[0].2.cmp(&b[0].2));
        slabs.iter_mut().for_each(|slab| {
            Self::drop_slab(slab, &height_map);
            Self::update_height_map(slab, &mut height_map);
        });
        Self { slabs }
    }

    fn solve_part_1(&self) -> Self::Part1Output {
        (0..self.slabs.len())
            .filter(|&idx| {
                let mut slabs = self.slabs.clone();
                slabs.remove(idx);
                let mut height_map = Grid::from_vec(vec![0; 100], 10);
                for slab in &slabs {
                    if slab
                        .iter()
                        .all(|(x, y, z)| height_map.get(&UPoint::new(*x, *y)) + 1 != *z)
                    {
                        return false;
                    }
                    Self::update_height_map(slab, &mut height_map);
                }
                true
            })
            .count()
    }

    fn solve_part_2(&self) -> Self::Part2Output {
        (0..self.slabs.len())
            .flat_map(|idx| {
                let mut slabs = self.slabs.clone();
                slabs.remove(idx);
                let mut height_map = Grid::from_vec(vec![0; 100], 10);

                slabs
                    .iter_mut()
                    .map(|slab| {
                        let n = Self::drop_slab(slab, &height_map) as usize;
                        Self::update_height_map(slab, &mut height_map);
                        n
                    })
                    .collect::<Vec<usize>>()
            })
            .sum::<usize>()
    }

    fn solution(
        &self,
        input_type: crate::solution::InputType,
    ) -> (Option<Self::Part1Output>, Option<Self::Part2Output>) {
        match input_type {
            InputType::Examples => (Some(5), Some(7)),
            InputType::Puzzles => (Some(426), Some(61920)),
        }
    }
}

impl Day22 {
    fn drop_slab(slab: &mut Slab, height_map: &Grid<usize>) -> bool {
        let mut changed = false;
        while slab
            .iter()
            .all(|(x, y, z)| height_map.get(&UPoint::new(*x, *y)) + 1 != *z)
        {
            changed = true;
            for tile in &mut *slab {
                tile.2 -= 1;
            }
        }
        changed
    }
    fn update_height_map(slab: &Slab, height_map: &mut Grid<usize>) {
        for tile in slab {
            let cur_height = height_map.get_mut(&UPoint::new(tile.0, tile.1));
            *cur_height = *cur_height.deref().max(&tile.2);
        }
    }
}
