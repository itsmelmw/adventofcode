use std::collections::HashSet;

fn parse(input: &str) -> Vec<Vec<usize>> {
    return input
        .split("\n")
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|b| match b {
                    b'a'..=b'z' => (b - b'a' + 1) as usize,
                    b'A'..=b'Z' => (b - b'A' + 27) as usize,
                    _ => unreachable!(),
                })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
}

fn solve1(parsed: &Vec<Vec<usize>>) -> String {
    return parsed
        .iter()
        .map(|sack| {
            let half = sack.len() / 2;
            let first = HashSet::<usize>::from_iter(sack[..half].iter().copied());
            let second = HashSet::<usize>::from_iter(sack[half..].iter().copied());
            *first.intersection(&second).next().unwrap()
        })
        .sum::<usize>()
        .to_string();
}

fn solve2(parsed: &Vec<Vec<usize>>) -> String {
    return parsed
        .chunks(3)
        .map(|group| {
            *group
                .iter()
                .map(|x| HashSet::<&usize>::from_iter(x.iter()))
                .reduce(|acc, val| acc.intersection(&val).copied().collect())
                .unwrap()
                .iter()
                .next()
                .unwrap()
        })
        .sum::<usize>()
        .to_string();
}

pub fn solve(input: &str) -> (String, String) {
    let parsed = parse(input);
    return (solve1(&parsed), solve2(&parsed));
}
