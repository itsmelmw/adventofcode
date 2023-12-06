use super::{CheckResult, Input, Part};
use itertools::Itertools;

pub struct DaySection {
    title: String,
    part1: CheckResult,
    part2: CheckResult,
}

impl DaySection {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            part1: Default::default(),
            part2: Default::default(),
        }
    }
    pub fn set_result(&mut self, part: &Part, result: CheckResult) {
        match part {
            Part::One => self.part1 = result,
            Part::Two => self.part2 = result,
        }
    }
}

pub struct DayTable {
    day: usize,
    sections: Vec<DaySection>,
}

impl DayTable {
    pub fn new(day: usize) -> Self {
        let sections = Input::iter()
            .map(|input| DaySection::new(input.name()))
            .collect();
        let new = Self { day, sections };
        new.print();
        new
    }

    pub fn set_result(&mut self, input_idx: usize, part: &Part, result: CheckResult) {
        self.sections[input_idx].set_result(part, result);
        self.print()
    }

    pub fn print(&self) {
        print!("\x1B[2J\x1B[1;1H");
        println!("╔══════════════════════════╗");
        println!("║ * Solutions for day {:02} * ║", self.day);
        for section in &self.sections {
            println!("╟{:─^26}╢", format!("[{:.22}]", section.title));
            println!(
                "║ {} Part 1: {:14.14} ║",
                section.part1.symbol(),
                section.part1.result(),
            );
            println!(
                "║ {} Part 2: {:14.14} ║",
                section.part2.symbol(),
                section.part2.result(),
            );
        }
        println!("╚══════════════════════════╝");
    }
}

pub struct YearTable {
    year: usize,
    days: [CheckResult; 25],
}

impl YearTable {
    pub fn new(year: usize) -> Self {
        let new = Self {
            year,
            days: Default::default(),
        };
        new.print();
        new
    }

    pub fn set_result(&mut self, day: usize, result: CheckResult) {
        self.days[day - 1] = result;
        self.print()
    }

    pub fn print(&self) {
        print!("\x1B[2J\x1B[1;1H");
        println!("╔═══════════════════╗");
        println!("║ * {:04} Calendar * ║", self.year);
        println!("╟───┬───┬───┬───┬───╢");
        let rows = self
            .days
            .chunks(5)
            .map(|row| {
                format!(
                    "║{}║\n",
                    row.iter()
                        .map(|day| format!(" {} ", day.symbol()))
                        .join("│")
                )
            })
            .join("╟───┼───┼───┼───┼───╢\n");
        print!("{}", rows);
        println!("╚═══╧═══╧═══╧═══╧═══╝");
    }
}
