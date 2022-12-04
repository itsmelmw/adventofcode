use std::env;
use std::fs;

mod solutions;
use solutions::*;

fn read_input(day: u8) -> String {
    let path = format!("src/input/day{:02}.txt", day);
    return fs::read_to_string(path).expect("Input file missing");
}

fn main() {
    if let Some(arg) = env::args().nth(1) {
        let day = arg.parse::<u8>();
        if let Ok((a, b)) = match day {
            Ok(d @ 1..=25) => {
                let input = read_input(d);
                match d {
                    1 => Ok(day01::solve(&input)),
                    2 => Ok(day02::solve(&input)),
                    3 => Ok(day03::solve(&input)),
                    4 => Ok(day04::solve(&input)),
                    _ => Err("No solution"),
                }
            }
            _ => Err("Invalid day"),
        } {
            println!("╔══════════════════════╗");
            println!("║ Solutions for day {:02} ║", arg);
            println!("╟──────────────────────╢");
            println!("║ Part 1: {:12} ║", a);
            println!("║ Part 2: {:12} ║", b);
            println!("╚══════════════════════╝");
        } else {
            println!("`{arg}` is not a valid day");
        }
    }
}
