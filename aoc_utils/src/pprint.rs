use std::{array, fmt};

use crate::solutions::{InputResult, Part, Solution};

#[derive(Clone, Copy)]
pub enum Symbol {
    Waiting,
    Correct,
    Wrong,
    Unknown,
}

impl Symbol {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Waiting => "\x1b[36m⋯\x1b[0m",
            Self::Correct => "\x1b[32m✔\x1b[0m",
            Self::Wrong => "\x1b[31m✘\x1b[0m",
            Self::Unknown => "\x1b[33m⁉\x1b[0m",
        }
    }
}

pub enum DayLine<'a> {
    Title(usize, &'a str),
    Header(&'a str),
    Result(Symbol, usize, &'a str),
    ResultExtra(&'a str),
    Top,
    Bottom,
}

impl<'a> DayLine<'a> {
    pub fn min_len(&self) -> usize {
        match self {
            Self::Title(_, title) => 10 + title.len(),
            Self::Header(header) => 2 + header.len(),
            Self::Result(_, _, result) => 12 + result.len(),
            Self::ResultExtra(result) => 12 + result.len(),
            _ => 0,
        }
    }
    pub fn as_str_with_len(&self, len: usize) -> String {
        match self {
            Self::Title(day, title) => {
                format!("║{:^len$}║", format!("Day {:02}: {}", day, title))
            }
            Self::Header(header) => {
                format!("╟{:─^len$}╢", format!("[{}]", header))
            }
            Self::Result(symbol, part, result) => {
                format!(
                    "║{:<w$}║",
                    format!(" {} Part {}: {}", symbol.as_str(), part, result),
                    w = len + 9
                )
            }
            Self::ResultExtra(result) => {
                format!("║{:<len$}║", format!("{}{}", " ".repeat(11), result))
            }
            Self::Top => {
                format!("╔{}╗", "═".repeat(len))
            }
            Self::Bottom => {
                format!("╚{}╝", "═".repeat(len))
            }
        }
    }
}

pub struct DayOverview<'a>(Vec<DayLine<'a>>);

impl<'a> DayOverview<'a> {
    pub fn from(day: usize, solution: &'a dyn Solution, results: &'a Vec<InputResult<'a>>) -> Self {
        let mut lines = vec![DayLine::Top, DayLine::Title(day, solution.title())];
        for result in results {
            lines.push(DayLine::Header(result.input().name()));
            for part in Part::iter() {
                let part_result = result.result(part);
                let symbol = get_symbol(part_result, solution.solution(result.input(), part));
                let mut res_lines = part_result
                    .as_ref()
                    .map(|s| s.as_str())
                    .unwrap_or("Waiting...")
                    .split('\n');
                lines.push(DayLine::Result(
                    symbol,
                    part.as_num(),
                    res_lines.next().unwrap(),
                ));
                for extra_line in res_lines {
                    lines.push(DayLine::ResultExtra(extra_line));
                }
            }
        }
        lines.push(DayLine::Bottom);
        DayOverview(lines)
    }
}

impl<'a> fmt::Display for DayOverview<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let len = self.0.iter().map(|line| line.min_len()).max().unwrap();
        for line in &self.0 {
            writeln!(f, "{}", line.as_str_with_len(len))?
        }
        Ok(())
    }
}

pub enum YearLine {
    Title(usize),
    DayHeaders([usize; 5], bool),
    DaySymbols([Symbol; 5]),
    TitleBottom,
    Top,
    Bottom,
}

impl YearLine {
    pub fn as_str(&self) -> String {
        match self {
            Self::Title(year) => {
                format!("║{:^24}║", format!("Advent of Code {:04}", year),)
            }
            Self::DayHeaders(days, top) => {
                let join_char = if *top { "┬" } else { "┼" };
                format!(
                    "╟{}╢",
                    days.map(|day| format!("[{:02}]", day)).join(join_char)
                )
            }
            Self::DaySymbols(symbols) => {
                format!(
                    "║{}║",
                    symbols
                        .map(|symbol| format!(" {}  ", symbol.as_str()))
                        .join("│")
                )
            }
            Self::TitleBottom => {
                format!("║{}║", " ".repeat(24))
            }
            Self::Top => {
                format!("╔{}╗", "═".repeat(24))
            }
            Self::Bottom => {
                format!("╚{}╝", ["════"; 5].join("╧"))
            }
        }
    }
}

pub struct YearOverview(Vec<YearLine>);

impl YearOverview {
    pub fn from<'a>(
        year: usize,
        solution: &'a dyn Solution,
        results: &'a [Vec<InputResult<'a>>],
    ) -> Self {
        let mut lines = vec![YearLine::Top, YearLine::Title(year), YearLine::TitleBottom];
        for (line_num, result_line) in results.chunks(5).enumerate() {
            let days = array::from_fn(|i| i + 1 + (line_num * 5));
            lines.push(YearLine::DayHeaders(days, line_num == 0));
            let symbols = days.map(|day| {
                for result in results[day - 1].iter() {
                    for part in Part::iter() {
                        let part_result = result.result(part);
                        match get_symbol(part_result, solution.solution(result.input(), part)) {
                            Symbol::Correct => (),
                            symbol => return symbol,
                        }
                    }
                }
                Symbol::Correct
            });
            lines.push(YearLine::DaySymbols(symbols));
        }
        // for line_num in 0..5 {
        //     lines.push(YearLine::DayHeaders(
        //         array::from_fn(|i| i + 1 + (line_num * 5)),
        //         line_num == 0,
        //     ));
        //     lines.push(YearLine::DaySymbols([Symbol::Waiting; 5]))
        // }
        lines.push(YearLine::Bottom);
        YearOverview(lines)
    }
}

impl fmt::Display for YearOverview {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in &self.0 {
            writeln!(f, "{}", line.as_str())?
        }
        Ok(())
    }
}

fn get_symbol(result: &Option<String>, truth: Option<&str>) -> Symbol {
    match (result, truth) {
        (None, _) => Symbol::Waiting,
        (Some(_), None) => Symbol::Unknown,
        (Some(v1), Some(v2)) if v1 == v2 => Symbol::Correct,
        (Some(_), Some(_)) => Symbol::Wrong,
    }
}
