// https://adventofcode.com/2022/day/10

use crate::solution::{InputType, Solution};

enum Instr {
    Noop,
    Addx(isize),
}

struct Cpu {
    cycle: isize,
    xreg: isize,
}

struct Crt {
    cpu: Cpu,
    dot: isize,
    display: String,
}

impl Cpu {
    fn new() -> Self {
        Self { cycle: 0, xreg: 1 }
    }

    fn step(&mut self, instr: &Instr) -> isize {
        match instr {
            Instr::Noop => self.cycle += 1,
            Instr::Addx(x) => {
                self.xreg += x;
                self.cycle += 1
            }
        }
        self.cycle
    }
}

impl Crt {
    fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            dot: 0,
            display: String::new(),
        }
    }

    fn get_pixel(&self) -> bool {
        self.dot >= (self.cpu.xreg - 1) && self.dot <= self.cpu.xreg + 1
    }

    fn step(&mut self, instr: &Instr) {
        self.display.push(if self.get_pixel() { '#' } else { '.' });
        self.dot += 1;
        if self.dot >= 40 {
            self.display.push('\n');
            self.dot = 0;
        }
        self.cpu.step(instr);
    }
}

pub struct Day10 {
    instrs: Vec<Instr>,
}

impl<'i> Solution<'i> for Day10 {
    type Part1Output = isize;
    type Part2Output = String;

    fn title(&self) -> &str {
        "Cathode-Ray Tube"
    }

    fn parse(input: &'i str) -> Self {
        let instrs = input
            .split('\n')
            .flat_map(|line| match line.split(' ').collect::<Vec<&str>>()[..] {
                // Add an extra NOOP in front of every ADDX to make it take 2 cycle.
                ["noop"] => vec![Instr::Noop],
                ["addx", x] => vec![Instr::Noop, Instr::Addx(x.parse::<isize>().unwrap())],
                _ => unreachable!(),
            })
            .collect::<Vec<Instr>>();
        Self { instrs }
    }

    fn solve_part_1(&self) -> Self::Part1Output {
        let mut cpu = Cpu::new();
        let mut val_cycle = 20_isize;

        self.instrs
            .iter()
            .filter_map(|instr| match cpu.step(instr) + 1 {
                c if c == val_cycle => {
                    val_cycle += 40;
                    Some(c * cpu.xreg)
                }
                _ => None,
            })
            .sum::<isize>()
    }

    fn solve_part_2(&self) -> Self::Part2Output {
        let mut crt = Crt::new();
        for instr in &self.instrs {
            crt.step(instr);
        }
        crt.display
    }

    fn solution(
        &self,
        input_type: crate::solution::InputType,
    ) -> (Option<Self::Part1Output>, Option<Self::Part2Output>) {
        match input_type {
            InputType::Examples => (Some(13140), Some("##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....\n".into())),
            InputType::Puzzles => (Some(14540), Some("####.#..#.####.####.####.#..#..##..####.\n#....#..#....#.#.......#.#..#.#..#....#.\n###..####...#..###....#..####.#......#..\n#....#..#..#...#.....#...#..#.#.....#...\n#....#..#.#....#....#....#..#.#..#.#....\n####.#..#.####.#....####.#..#..##..####.\n".into())),
        }
    }
}
