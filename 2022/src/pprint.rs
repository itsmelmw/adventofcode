pub struct Output {
    pub title: &'static str,
    pub result: Result<(String, String), &'static str>,
    pub truth: (&'static str, &'static str),
}

fn get_truth_symbol(result: &String, truth: &str) -> &'static str {
    match truth {
        "" => "\x1b[33m?\x1b[0m",
        _ if result == truth => "\x1b[32m✔\x1b[0m",
        _ => "\x1b[31m✘\x1b[0m",
    }
}

fn print_day_header(day: usize) {
    println!("╔══════════════════════════╗");
    println!("║ * Solutions for day {:02} * ║", day);
}

fn print_day_parts(solution: &Output) {
    println!("╟{:─^26}╢", format!("[{:.22}]", solution.title));
    match &solution.result {
        Ok(res) => {
            println!(
                "║ {} Part 1: {:14.14} ║",
                get_truth_symbol(&res.0, solution.truth.0),
                res.0,
            );
            println!(
                "║ {} Part 2: {:14.14} ║",
                get_truth_symbol(&res.1, solution.truth.1),
                res.1,
            );
        }
        Err(err) => println!("║ {:24} ║", err),
    }
}

fn print_day_footer() {
    println!("╚══════════════════════════╝");
}

fn print_day_too_long(solution: &Output) {
    if let Ok(result) = &solution.result {
        if result.0.len() > 14 {
            println!(
                "Output of {} Part 1 was too large! Full output:\n{}",
                solution.title, result.0
            );
        }
        if result.1.len() > 14 {
            println!(
                "Output of {} Part 2 was too large! Full output:\n{}",
                solution.title, result.1
            );
        }
    }
}

pub fn pprint_day_solutions(day: usize, solutions: Vec<Output>) {
    print_day_header(day);
    for solution in &solutions {
        print_day_parts(solution);
    }
    print_day_footer();
    for solution in &solutions {
        print_day_too_long(solution);
    }
}

fn print_all_header(year: usize) {
    println!("╔═══════════════════╗");
    println!("║ * {:04} Calendar * ║", year);
    println!("╟───┬───┬───┬───┬───╢");
}

fn print_all_result_row() {
    println!("║   │   │   │   │   ║")
}

fn print_all_divider() {
    println!("╟───┼───┼───┼───┼───╢")
}

fn print_all_footer() {
    println!("╚═══╧═══╧═══╧═══╧═══╝");
}

pub fn pprint_all_solutions(year: usize, _solutions: Vec<Vec<Output>>) {
    print_all_header(year);
    print_all_result_row();
    print_all_divider();
    print_all_result_row();
    print_all_divider();
    print_all_result_row();
    print_all_divider();
    print_all_result_row();
    print_all_divider();
    print_all_result_row();
    print_all_footer();
}
