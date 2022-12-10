// https://adventofcode.com/2022/day/10

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
        return Self { cycle: 0, xreg: 1 };
    }

    fn step(&mut self, instr: &Instr) -> isize {
        match instr {
            Instr::Noop => self.cycle += 1,
            Instr::Addx(x) => {
                self.xreg += x;
                self.cycle += 1
            }
        }
        return self.cycle;
    }
}

impl Crt {
    fn new() -> Self {
        return Self {
            cpu: Cpu::new(),
            dot: 0,
            display: String::new(),
        };
    }

    fn get_pixel(&self) -> bool {
        return self.dot >= (self.cpu.xreg - 1) && self.dot <= self.cpu.xreg + 1;
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

fn parse(input: &str) -> Vec<Instr> {
    return input
        .split("\n")
        .flat_map(|line| match line.split(" ").collect::<Vec<&str>>()[..] {
            // Add an extra NOOP in front of every ADDX to make it take 2 cycle.
            ["noop"] => vec![Instr::Noop],
            ["addx", x] => vec![Instr::Noop, Instr::Addx(x.parse::<isize>().unwrap())],
            _ => unreachable!(),
        })
        .collect::<Vec<Instr>>();
}

fn solve1(parsed: &Vec<Instr>) -> String {
    let mut cpu = Cpu::new();
    let mut val_cycle = 20 as isize;

    return parsed
        .iter()
        .filter_map(|instr| match cpu.step(instr) + 1 {
            c if c == val_cycle => {
                val_cycle += 40;
                Some(c * cpu.xreg)
            }
            _ => None,
        })
        .sum::<isize>()
        .to_string();
}

fn solve2(parsed: &Vec<Instr>) -> String {
    let mut crt = Crt::new();
    for instr in parsed {
        crt.step(instr);
    }
    return crt.display;
}

pub fn solve(input: &str) -> (String, String) {
    let parsed = parse(input);
    return (solve1(&parsed), solve2(&parsed));
}
