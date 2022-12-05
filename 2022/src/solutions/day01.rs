// https://adventofcode.com/2022/day/1

struct Top3 {
    list: [usize; 3],
    min_ind: usize,
}

impl Top3 {
    fn init() -> Self {
        return Top3 {
            list: [0, 0, 0],
            min_ind: 0,
        };
    }

    fn sum(&self) -> usize {
        return self.list.iter().sum();
    }

    fn update_min(&mut self) {
        self.min_ind = 0;
        for i in [1, 2] {
            if self.list[i] < self.list[self.min_ind] {
                self.min_ind = i;
            }
        }
    }

    fn update(&mut self, new: usize) {
        if new > self.list[self.min_ind] {
            self.list[self.min_ind] = new;
            self.update_min();
        }
    }
}

fn parse(input: &str) -> Vec<usize> {
    return input
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .map(|cal| cal.parse::<usize>().unwrap())
                .sum()
        })
        .collect::<Vec<usize>>();
}

fn solve1(parsed: &Vec<usize>) -> String {
    return parsed.iter().max().unwrap().to_string();
}

fn solve2(parsed: &Vec<usize>) -> String {
    let mut top3 = Top3::init();
    for &value in parsed.iter() {
        top3.update(value);
    }
    return top3.sum().to_string();
}

pub fn solve(input: &str) -> (String, String) {
    let parsed = parse(input);
    return (solve1(&parsed), solve2(&parsed));
}
