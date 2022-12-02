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
            Ok(1) => Ok(day01::solve(read_input(1))),
            Ok(2) => Ok(day02::solve(read_input(2))),

            _ => Err(""),
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
