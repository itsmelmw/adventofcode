// https://adventofcode.com/2022/day/8

use itertools::Itertools;
use std::collections::HashSet;

fn parse(input: &str) -> (Vec<usize>, usize) {
    // Assume the input is square, so the amount
    // of lines is equal to the dimensions.
    let mut lines = 1;
    let parsed = input
        .as_bytes()
        .iter()
        .filter_map(|x| match x {
            b'\n' => {
                lines += 1;
                None
            }
            _ => Some((x - b'0') as usize),
        })
        .collect();
    return (parsed, lines);
}

fn insert_visible<'a, I>(treeline: I, counted: &mut HashSet<*const usize>)
where
    I: Iterator<Item = &'a usize>,
{
    let mut highest = None;
    for tree in treeline {
        if highest.is_none() || tree > highest.unwrap() {
            highest = Some(tree);
            counted.insert(tree as *const usize);
            if let Some(9) = highest {
                break;
            }
        }
    }
}

fn count_visible<'a, I>(treeline: I, init_size: &usize) -> usize
where
    I: Iterator<Item = &'a usize>,
{
    let mut count = 0;
    for tree in treeline {
        count += 1;
        if tree >= init_size {
            break;
        }
    }
    return count;
}

fn scenic_score(parsed: &Vec<usize>, dim: usize, x: usize, y: usize) -> usize {
    let idx = y * dim + x;
    let size = &parsed[idx];

    let to_left = parsed.iter().skip(y * dim).take(x).rev();
    let to_right = parsed.iter().skip(idx + 1).take(dim - x - 1);
    let to_top = parsed.iter().skip(x).step_by(dim).take(y).rev();
    let to_bottom = parsed.iter().skip(idx + dim).step_by(dim).take(dim - y - 1);

    return count_visible(to_left, size)
        * count_visible(to_right, size)
        * count_visible(to_top, size)
        * count_visible(to_bottom, size);
}

fn solve1<'a>(parsed: &'a Vec<usize>, dim: usize) -> String {
    // Create a hashset of pointers, to check whether we counted a tree before
    let mut found = HashSet::<*const usize>::new();

    // Horizontal
    for y in 0..dim {
        let h_iter = parsed.iter().skip(y * dim).take(dim);
        insert_visible(h_iter.clone(), &mut found);
        insert_visible(h_iter.rev(), &mut found);
    }

    // Vectical
    for x in 0..dim {
        let v_iter = parsed.iter().skip(x).step_by(dim);
        insert_visible(v_iter.clone(), &mut found);
        insert_visible(v_iter.rev(), &mut found);
    }

    return found.len().to_string();
}

fn solve2(parsed: &Vec<usize>, dim: usize) -> String {
    return (0..dim)
        .cartesian_product(0..dim)
        .map(|(x, y)| scenic_score(parsed, dim, x, y))
        .max()
        .unwrap()
        .to_string();
}

pub fn solve(input: &str) -> (String, String) {
    let (parsed, dim) = parse(input);
    return (solve1(&parsed, dim), solve2(&parsed, dim));
}
