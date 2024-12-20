mod grids;
mod solution;
mod year2022;
mod year2023;
mod year2024;

use clap::Parser;
use solution::Year;
use year2022::Year2022;
use year2023::Year2023;
use year2024::Year2024;

#[derive(Parser)]
struct Opts {
    #[arg(required = true)]
    day: usize,
    #[arg(short, long, default_value = "2024")]
    year: usize,
    #[arg(short, long, action)]
    example: bool,
}

fn main() {
    let opts = Opts::parse();
    let input_type = if opts.example {
        solution::InputType::Examples
    } else {
        solution::InputType::Puzzles
    };
    match opts.year {
        2022 => Year2022.solve(opts.day, input_type),
        2023 => Year2023.solve(opts.day, input_type),
        2024 => Year2024.solve(opts.day, input_type),
        _ => unimplemented!(),
    }
}
