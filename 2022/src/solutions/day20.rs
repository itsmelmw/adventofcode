// https://adventofcode.com/2022/day/20

fn parse(input: &str) -> Vec<isize> {
    return input
        .split("\n")
        .map(|line| line.parse().unwrap())
        .collect();
}

fn solve1(parsed: &Vec<isize>) -> String {
    let mut mixed = parsed.clone();
    let mut idx_map = (0..parsed.len()).collect::<Vec<usize>>();

    parsed.iter().enumerate().for_each(|(i, &num)| {
        let mix_idx = idx_map[i];
        let val = mixed.remove(mix_idx);
        let new_idx = ((mix_idx as isize) + num).rem_euclid((parsed.len() - 1) as isize) as usize;
        dbg!(num);
        dbg!(mix_idx);
        dbg!((mix_idx as isize) + num);
        dbg!(new_idx);
        mixed.insert(new_idx, val);
        let val = idx_map.remove(mix_idx);
        idx_map.insert(new_idx, val);

        dbg!(&mixed);
        dbg!(&idx_map);
    });
    return 0.to_string();
}

fn solve2(parsed: &Vec<isize>) -> String {
    return 0.to_string();
}

pub fn solve(input: &str) -> (String, String) {
    let parsed = parse(input);
    return (solve1(&parsed), solve2(&parsed));
}
