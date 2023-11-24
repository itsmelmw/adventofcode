// https://adventofcode.com/2022/day/22

use crate::solutions::{InputParser, ProblemSolver};

type Map2d = Vec<Vec<Tile>>;
type Instructions = Vec<Instruction>;

struct State {
    x: usize,
    y: usize,
    dir: Dir,
}

#[derive(PartialEq)]
enum Tile {
    Empty,
    Wall,
    Wrap,
}

enum Instruction {
    Step(usize),
    Turn(Turn),
}

enum Turn {
    Left,
    Right,
}

enum Dir {
    Up,
    Right,
    Down,
    Left,
}
impl Dir {
    pub fn turn(&self, turn_dir: Turn) -> Self {
        let val = self.as_u8();
        match turn_dir {
            Turn::Left => Dir::from_u8((val + 3) % 4),
            Turn::Right => Dir::from_u8((val + 1) % 4),
        }
    }
    pub fn as_u8(&self) -> u8 {
        match self {
            Dir::Up => 0,
            Dir::Right => 1,
            Dir::Down => 2,
            Dir::Left => 3,
        }
    }
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => Dir::Up,
            1 => Dir::Right,
            2 => Dir::Down,
            3 => Dir::Left,
            _ => unreachable!(),
        }
    }
}

fn parse_instrs(input: &str) -> Instructions {
    let mut parsed_instrs = Vec::new();
    let mut char_iter = input.chars().peekable();
    while let Some(c) = char_iter.next() {
        let instr = match c {
            'L' => Instruction::Turn(Turn::Left),
            'R' => Instruction::Turn(Turn::Right),
            n => {
                let mut nums = n.to_string();
                while let Some(c) = char_iter.peek() {
                    if !c.is_numeric() {
                        break;
                    }
                    nums.push(char_iter.next().unwrap());
                }
                Instruction::Step(nums.parse::<usize>().unwrap())
            }
        };
        parsed_instrs.push(instr);
    }
    parsed_instrs
}

fn parse_map(input: &str) -> Map2d {
    input
        .split('\n')
        .map(|row| {
            row.chars()
                .map(|c| match c {
                    ' ' => Tile::Wrap,
                    '.' => Tile::Empty,
                    '#' => Tile::Wall,
                    _ => unreachable!(),
                })
                .collect::<Vec<Tile>>()
        })
        .collect::<Map2d>()
}

fn parse(input: &str) -> (Map2d, Instructions) {
    let (map, instrs) = input.split_once("\n\n").unwrap();
    (parse_map(map), parse_instrs(instrs))
}

pub struct Parser;

impl InputParser for Parser {
    type S = Solver;
    fn parse(input: &str) -> Solver {
        let (map, instructions) = parse(input);
        Solver { map, instructions }
    }
}

pub struct Solver {
    map: Map2d,
    instructions: Instructions,
}

impl ProblemSolver for Solver {
    fn solve_part_1(&self) -> String {
        0.to_string()
    }
    fn solve_part_2(&self) -> String {
        0.to_string()
    }
}
