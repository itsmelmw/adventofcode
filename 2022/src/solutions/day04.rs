// https://adventofcode.com/2022/day/4

use itertools::Itertools;

type Assignment = ((usize, usize), (usize, usize));

fn contains(range1: (usize, usize), range2: (usize, usize)) -> bool {
    (range1.0 >= range2.0 && range1.1 <= range2.1) || (range2.0 >= range1.0 && range2.1 <= range1.1)
}

fn overlaps(range1: (usize, usize), range2: (usize, usize)) -> bool {
    (range1.0 >= range2.0 && range1.0 <= range2.1) || (range2.0 >= range1.0 && range2.0 <= range1.1)
}

fn parse(input: &str) -> Vec<((usize, usize), (usize, usize))> {
    return input
        .split('\n')
        .map(|line| {
            line.split(',')
                .map(|range| {
                    range
                        .splitn(2, '-')
                        .map(|num| num.parse::<usize>().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect_tuple()
                .unwrap()
        })
        .collect::<Vec<Assignment>>();
}

fn solve1(parsed: &[Assignment]) -> String {
    return parsed
        .iter()
        .map(|ranges| contains(ranges.0, ranges.1) as usize)
        .sum::<usize>()
        .to_string();
}

fn solve2(parsed: &[Assignment]) -> String {
    return parsed
        .iter()
        .map(|ranges| overlaps(ranges.0, ranges.1) as usize)
        .sum::<usize>()
        .to_string();
}

pub fn solve(input: &str) -> (String, String) {
    let parsed = parse(input);
    (solve1(&parsed), solve2(&parsed))
}
