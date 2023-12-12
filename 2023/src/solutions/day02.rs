// https://adventofcode.com/2023/day/2

use aoc_utils::solutions::{InputDir, Part, Solution};

struct Game {
    id: usize,
    cubes: Vec<Cubes>,
}

struct Cubes {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

pub struct Day02 {
    games: Vec<Game>,
}

impl Solution for Day02 {
    fn title(&self) -> &str {
        "Cube Conundrum"
    }
    fn parse(input: &str) -> Self {
        let games = input
            .split('\n')
            .map(|line| {
                let (prefix, game) = line.split_once(": ").unwrap();
                let id = prefix.split_once(' ').unwrap().1.parse::<usize>().unwrap();
                let cubes = game
                    .split("; ")
                    .map(|cubes_str| {
                        let mut cube_struct = Cubes {
                            red: 0,
                            green: 0,
                            blue: 0,
                        };
                        cubes_str.split(", ").for_each(|cube_str| {
                            match cube_str.split_once(' ').unwrap() {
                                (num, "red") => cube_struct.red = num.parse::<usize>().unwrap(),
                                (num, "green") => cube_struct.green = num.parse::<usize>().unwrap(),
                                (num, "blue") => cube_struct.blue = num.parse::<usize>().unwrap(),
                                _ => unreachable!(),
                            }
                        });
                        cube_struct
                    })
                    .collect::<Vec<Cubes>>();
                Game { id, cubes }
            })
            .collect::<Vec<Game>>();
        Self { games }
    }
    fn solve_part_1(&self) -> String {
        let red_max = 12;
        let green_max = 13;
        let blue_max = 14;
        self.games
            .iter()
            .filter_map(|game| {
                if game.cubes.iter().all(|cubes| {
                    cubes.red <= red_max && cubes.green <= green_max && cubes.blue <= blue_max
                }) {
                    Some(game.id)
                } else {
                    None
                }
            })
            .sum::<usize>()
            .to_string()
    }
    fn solve_part_2(&self) -> String {
        self.games
            .iter()
            .map(|game| {
                game.cubes.iter().map(|c| c.red).max().unwrap()
                    * game.cubes.iter().map(|c| c.green).max().unwrap()
                    * game.cubes.iter().map(|c| c.blue).max().unwrap()
            })
            .sum::<usize>()
            .to_string()
    }
    fn answer(&self, input: &InputDir, part: &Part) -> Option<&str> {
        match (input.name().as_str(), part) {
            ("Example", Part::One) => Some("8"),
            ("Example", Part::Two) => Some("2286"),
            ("Puzzle", Part::One) => Some("2776"),
            ("Puzzle", Part::Two) => Some("68638"),
            _ => unreachable!(),
        }
    }
}
