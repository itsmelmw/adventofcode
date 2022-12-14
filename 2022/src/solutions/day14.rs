// https://adventofcode.com/2022/day/14

use itertools::Itertools;
use std::collections::HashSet;

fn parse(input: &str) -> (HashSet<(usize, usize)>, usize) {
    let mut points = HashSet::new();
    let mut lowest = 0;
    input.split("\n").for_each(|line| {
        line.split(" -> ")
            .map(|pt| {
                pt.split(",")
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect_tuple::<(usize, usize)>()
                    .unwrap()
            })
            .tuple_windows::<((usize, usize), (usize, usize))>()
            .for_each(|(pt1, pt2)| {
                if pt1.1 > lowest {
                    lowest = pt1.1;
                }
                let mut pt = pt1;
                let xdir = ((pt2.0 as isize) - (pt1.0 as isize)).clamp(-1, 1);
                let ydir = ((pt2.1 as isize) - (pt1.1 as isize)).clamp(-1, 1);
                while pt != pt2 {
                    points.insert(pt);
                    pt.0 = (pt.0 as isize + xdir) as usize;
                    pt.1 = (pt.1 as isize + ydir) as usize;
                }
                points.insert(pt2);
            })
    });

    return (points, lowest);
}

fn drop_sand(points: &HashSet<(usize, usize)>, floor: usize) -> (usize, usize) {
    let mut sand = (500, 0);
    loop {
        if sand.1 == floor {
            break;
        } else if !points.contains(&(sand.0, sand.1 + 1)) {
            sand = (sand.0, sand.1 + 1);
        } else if !points.contains(&(sand.0 - 1, sand.1 + 1)) {
            sand = (sand.0 - 1, sand.1 + 1);
        } else if !points.contains(&(sand.0 + 1, sand.1 + 1)) {
            sand = (sand.0 + 1, sand.1 + 1);
        } else {
            break;
        }
    }
    return sand;
}

fn solve1(points: &mut HashSet<(usize, usize)>, abyss: usize) -> String {
    let orig_size = points.len();
    loop {
        let sand = drop_sand(points, abyss);
        if sand.1 == abyss {
            break;
        }
        points.insert(sand);
    }
    return (points.len() - orig_size).to_string();
}

fn solve2(points: &mut HashSet<(usize, usize)>, floor: usize) -> String {
    let orig_size = points.len();
    loop {
        let sand = drop_sand(points, floor);
        points.insert(sand);
        if sand == (500, 0) {
            break;
        }
    }
    return (points.len() - orig_size).to_string();
}

pub fn solve(input: &str) -> (String, String) {
    let (mut parsed, lowest) = parse(input);
    let mut parsed2 = parsed.clone();
    return (
        solve1(&mut parsed, lowest),
        solve2(&mut parsed2, lowest + 1),
    );
}
