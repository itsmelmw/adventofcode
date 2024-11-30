// https://adventofcode.com/2023/day/2

use crate::solution::{InputType, Day};

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

impl<'i> Day<'i> for Day02 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn title(&self) -> &str {
        "Cube Conundrum"
    }

    fn parse(input: &'i str) -> Self {
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

    fn solve_part_1(&self) -> Self::Part1Output {
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
    }

    fn solve_part_2(&self) -> Self::Part2Output {
        self.games
            .iter()
            .map(|game| {
                game.cubes.iter().map(|c| c.red).max().unwrap()
                    * game.cubes.iter().map(|c| c.green).max().unwrap()
                    * game.cubes.iter().map(|c| c.blue).max().unwrap()
            })
            .sum::<usize>()
    }

    fn solution(
        &self,
        input_type: crate::solution::InputType,
    ) -> (Option<Self::Part1Output>, Option<Self::Part2Output>) {
        match input_type {
            InputType::Examples => (Some(8), Some(2286)),
            InputType::Puzzles => (Some(2776), Some(68638)),
        }
    }
}
