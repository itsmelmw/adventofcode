// https://adventofcode.com/2022/day/13

use itertools::{EitherOrBoth, Itertools};
use std::cmp::{Ordering, PartialOrd};
use std::str::Chars;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Element {
    List(Vec<Element>),
    Num(usize),
}

impl Element {
    fn to_singleton(&self) -> Element {
        Element::List(vec![self.clone()])
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Element) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Element) -> Ordering {
        match (self, other) {
            (Element::Num(left), Element::Num(right)) => left.cmp(right),
            (left @ Element::Num(_), right @ Element::List(_)) => left.to_singleton().cmp(right),
            (left @ Element::List(_), right @ Element::Num(_)) => left.cmp(&right.to_singleton()),
            (Element::List(left), Element::List(right)) => left
                .iter()
                .zip_longest(right)
                .find_map(|zipped| match zipped {
                    EitherOrBoth::Both(l, r) => match l.cmp(r) {
                        Ordering::Equal => None,
                        o => Some(o),
                    },
                    EitherOrBoth::Left(_) => Some(Ordering::Greater),
                    EitherOrBoth::Right(_) => Some(Ordering::Less),
                })
                .unwrap_or(Ordering::Equal),
        }
    }
}

fn parse_list(chars: &mut Chars) -> Element {
    let mut list = Vec::new();
    let mut nums = String::new();
    while let Some(char) = chars.next() {
        match char {
            '[' => list.push(parse_list(chars)),
            ']' => {
                if !nums.is_empty() {
                    list.push(Element::Num(nums.parse::<usize>().unwrap()));
                    nums.clear();
                }
                return Element::List(list);
            }
            ',' => {
                if !nums.is_empty() {
                    list.push(Element::Num(nums.parse::<usize>().unwrap()));
                    nums.clear();
                }
            }
            num => nums.push(num),
        }
    }
    unreachable!();
}

fn parse(input: &str) -> Vec<Element> {
    return input
        .split('\n')
        .filter_map(|elem| match elem {
            "" => None,
            _ => Some(parse_list(&mut elem[1..].chars())),
        })
        .collect::<Vec<Element>>();
}

fn solve1(parsed: &[Element]) -> String {
    return parsed
        .iter()
        .tuples::<(&Element, &Element)>()
        .enumerate()
        .map(|(i, (l, r))| if l < r { i + 1 } else { 0 })
        .sum::<usize>()
        .to_string();
}

fn solve2(parsed: &mut Vec<Element>) -> String {
    let mut key = 1;
    let div_packet_1 = &Element::List(vec![Element::List(vec![Element::Num(2)])]);
    let div_packet_2 = &Element::List(vec![Element::List(vec![Element::Num(6)])]);
    parsed.push(div_packet_1.clone());
    parsed.push(div_packet_2.clone());

    parsed
        .iter()
        .sorted()
        .enumerate()
        .for_each(|(i, p)| match p {
            // Could probably be faster with some pointer magic, but this is fine.
            _ if p == div_packet_1 || p == div_packet_2 => key *= i + 1,
            _ => (),
        });

    key.to_string()
}

pub fn solve(input: &str) -> (String, String) {
    let mut parsed = parse(input);
    (solve1(&parsed), solve2(&mut parsed))
}
