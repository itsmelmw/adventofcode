use std::fmt;

use crate::solutions::{InputResult, Part, Solution};

#[derive(Clone)]
pub enum Symbol {
    Waiting,
    Correct,
    Wrong,
    Unknown,
    // Error,
}

impl Symbol {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Waiting => "\x1b[36m…\x1b[0m",
            Self::Correct => "\x1b[32m✔\x1b[0m",
            Self::Wrong => "\x1b[31m✘\x1b[0m",
            Self::Unknown => "\x1b[33m?\x1b[0m",
            // Self::Error => "\x1b[36m!\x1b[0m",
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
                format!("║{:<len$}║", format!("           {}", result))
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
    pub fn from(
        day: usize,
        solution: &'a Box<dyn Solution>,
        results: &'a Vec<InputResult<'a>>,
    ) -> Self {
        let mut lines = vec![DayLine::Top, DayLine::Title(day, solution.title())];
        for result in results {
            lines.push(DayLine::Header(result.input().name()));
            for part in Part::iter() {
                let part_result = result.result(part);
                let symbol = Self::get_symbol(part_result, solution.solution(result.input(), part));
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
    fn get_symbol(result: &Option<String>, truth: Option<&str>) -> Symbol {
        match (result, truth) {
            (None, _) => Symbol::Waiting,
            (Some(_), None) => Symbol::Unknown,
            (Some(v1), Some(v2)) if v1 == v2 => Symbol::Correct,
            (Some(_), Some(_)) => Symbol::Wrong,
        }
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
