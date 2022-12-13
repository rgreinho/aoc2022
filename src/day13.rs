use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, u32},
    combinator::map,
    error::Error,
    multi::separated_list0,
    sequence::{delimited, separated_pair},
    Finish, IResult, Parser,
};
use std::{
    cmp::Ordering::{Equal, Greater, Less},
    fs,
    str::FromStr,
};

pub fn day13a() -> String {
    let data = fs::read_to_string("assets/day13.txt").expect("Could not load file");
    let pairs = parse_input_a(&data);
    let sum = process_input_a(&pairs);
    sum.to_string()
}

pub fn day13b() -> String {
    let data = fs::read_to_string("assets/day13.txt").expect("Could not load file");
    let pairs = parse_input_a(&data);
    let sum = process_input_b(&pairs);
    sum.to_string()
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Pair {
    left: Packet,
    right: Packet,
}

impl FromStr for Pair {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Pair::parse(s).finish() {
            Ok((_, item)) => Ok(item),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

impl Pair {
    pub fn parse(i: &str) -> IResult<&str, Pair> {
        map(
            separated_pair(Packet::parse, newline, Packet::parse),
            |(left, right)| Pair { left, right },
        )(i)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Packet {
    List(Vec<Packet>),
    Number(u32),
}

impl FromStr for Packet {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Packet::parse(s).finish() {
            Ok((_, item)) => Ok(item),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

impl PartialOrd for Packet {
    fn lt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Less))
    }

    fn le(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Less | Equal))
    }

    fn gt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Greater))
    }

    fn ge(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Greater | Equal))
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::List(a), Packet::List(b)) => a.cmp(b),
            (Packet::List(a), Packet::Number(b)) => a.cmp(&vec![Packet::Number(*b)]),
            (Packet::Number(a), Packet::List(b)) => vec![Packet::Number(*a)].cmp(&b),
            (Packet::Number(a), Packet::Number(b)) => a.cmp(b),
        }
    }
}

impl Packet {
    pub fn parse(i: &str) -> IResult<&str, Packet> {
        alt((
            delimited(tag("["), separated_list0(tag(","), Packet::parse), tag("]"))
                .map(|vec| Packet::List(vec)),
            u32.map(|num| Packet::Number(num)),
        ))(i)
    }
}

pub fn parse_input_a(input: &str) -> Vec<Pair> {
    input
        .split("\n\n")
        .map(String::from)
        .map(|l| l.parse::<Pair>().unwrap())
        .collect()
}

pub fn process_input_a(pairs: &[Pair]) -> usize {
    pairs
        .iter()
        .enumerate()
        .filter_map(|(i, Pair { left, right })| match left.cmp(&right) {
            Less => Some(i),
            Equal => panic!("Comparison cannot be equal!"),
            Greater => None,
        })
        .map(|v| v + 1)
        .sum::<usize>()
}

pub fn process_input_b(pairs: &[Pair]) -> usize {
    let mut ordered_packets: Vec<Packet> = Vec::new();
    for pair in pairs {
        ordered_packets.push(pair.left.clone());
        ordered_packets.push(pair.right.clone());
    }
    let dp2 = Packet::List(vec![Packet::Number(2)]);
    let dp6 = Packet::List(vec![Packet::Number(6)]);
    ordered_packets.push(dp2);
    ordered_packets.push(dp6);
    ordered_packets.sort();
    let dp2_pos = ordered_packets
        .iter()
        .position(|p| *p == Packet::List(vec![Packet::Number(2)]))
        .unwrap();
    let dp6_pos = ordered_packets
        .iter()
        .position(|p| *p == Packet::List(vec![Packet::Number(6)]))
        .unwrap();
    (dp2_pos + 1) * (dp6_pos + 1)
}

#[cfg(test)]
mod test {
    use super::*;

    const RAW_INPUT: &'static str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

    #[test]
    fn test_day13a_sample() {
        let pairs = parse_input_a(RAW_INPUT);
        let sum = process_input_a(&pairs);
        assert_eq!(sum, 13);
    }

    #[test]
    fn test_day13b_sample() {
        let pairs = parse_input_a(RAW_INPUT);
        let decoder_key = process_input_b(&pairs);
        assert_eq!(decoder_key, 140);
    }
}
