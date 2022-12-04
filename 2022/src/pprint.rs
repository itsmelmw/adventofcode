pub struct Output {
    pub title: &'static str,
    pub result: Result<(String, String), &'static str>,
    pub truth: (&'static str, &'static str),
}

fn print_header(day: usize) {
    println!("╔══════════════════════════╗");
    println!("║ * Solutions for day {:02} * ║", day);
}

fn get_truth_symbol(result: &String, truth: &str) -> &'static str {
    return match truth {
        "" => "\x1b[33m?\x1b[0m",
        _ if result == truth => "\x1b[32m✔\x1b[0m",
        _ => "\x1b[31m✘\x1b[0m",
    };
}

fn print_parts(solution: Output) {
    println!("╟{:─^26}╢", format!("[{:.22}]", solution.title));
    match solution.result {
        Ok(res) => {
            println!(
                "║ {} Part 1: {:14} ║",
                get_truth_symbol(&res.0, solution.truth.0),
                res.0,
            );
            println!(
                "║ {} Part 2: {:14} ║",
                get_truth_symbol(&res.1, solution.truth.1),
                res.1,
            );
        }
        Err(err) => println!("║ {:24} ║", err),
    }
}

fn print_footer() {
    println!("╚══════════════════════════╝");
}

pub fn pprint_solutions(day: usize, solutions: Vec<Output>) {
    print_header(day);
    for solution in solutions {
        print_parts(solution);
    }
    print_footer();
}
