use std::{collections::HashSet, fs, str::FromStr};

use nom::{
    character::complete::{char, u32},
    combinator::map,
    error::Error,
    sequence::separated_pair,
    Finish, IResult,
};

pub fn day04a() -> String {
    let data = fs::read_to_string("assets/day04.txt").expect("Could not load file");
    let pairs = parse_input_a(&data);
    let overlaps: u32 = process_input_a(&pairs);
    overlaps.to_string()
}

pub fn day04b() -> String {
    let data = fs::read_to_string("assets/day04.txt").expect("Could not load file");
    let pairs = parse_input_a(&data);
    let partial_overlaps: u32 = process_input_b(&pairs);
    partial_overlaps.to_string()
}

#[derive(Debug)]
pub struct Assignement {
    section: HashSet<u32>,
}

// impl From<&str> for Assignement {
//     fn from(item: &str) -> Self {
//         let v: Vec<&str> = item.split('-').collect();
//         if v.len() != 2 {
//             panic!("Invalid entry: {item}.")
//         }
//         let start = v.first().unwrap().parse::<u32>().unwrap();
//         let end = v.last().unwrap().parse::<u32>().unwrap();
//         let section: HashSet<u32> = (start..=end).collect();
//         Assignement { section }
//     }
// }

impl Assignement {
    pub fn parse(i: &str) -> IResult<&str, Assignement> {
        map(separated_pair(u32, char('-'), u32), |(start, end)| {
            Assignement {
                section: (start..=end).collect(),
            }
        })(i)
    }
}

impl FromStr for Assignement {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Assignement::parse(s).finish() {
            Ok((_, item)) => Ok(item),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

#[derive(Debug)]
pub struct Pair(Assignement, Assignement);

// impl From<&str> for Pair {
//     fn from(item: &str) -> Self {
//         let v: Vec<&str> = item.split(',').collect();
//         if v.len() != 2 {
//             panic!("Invalid entry: {item}.")
//         }
//         let s1: Assignement = v
//             .first()
//             .map(|&e| Assignement::from_str(e).unwrap())
//             .unwrap();
//         let s2: Assignement = v
//             .last()
//             .map(|&e| Assignement::from_str(e).unwrap())
//             .unwrap();
//         Pair(s1, s2)
//     }
// }

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
            separated_pair(Assignement::parse, char(','), Assignement::parse),
            |(left, right)| Pair(left, right),
        )(i)
    }
    pub fn overlap(&self) -> bool {
        self.0.section.is_superset(&self.1.section) || self.1.section.is_superset(&self.0.section)
    }

    pub fn partial_overlap(&self) -> bool {
        let intersection: HashSet<_> = self.0.section.intersection(&self.1.section).collect();
        !intersection.is_empty()
    }
}

pub fn parse_input_a(input: &str) -> Vec<Pair> {
    input.lines().map(|l| l.parse::<Pair>().unwrap()).collect()
}

pub fn process_input_a(pairs: &[Pair]) -> u32 {
    pairs.iter().map(|p| if p.overlap() { 1 } else { 0 }).sum()
}

pub fn process_input_b(pairs: &[Pair]) -> u32 {
    pairs
        .iter()
        .map(|p| if p.partial_overlap() { 1 } else { 0 })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const RAW_INPUT: &'static str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

    #[test]
    fn test_day04a_sample() {
        let pairs = parse_input_a(RAW_INPUT);
        let overlaps: u32 = process_input_a(&pairs);
        assert_eq!(overlaps, 2)
    }

    #[test]
    fn test_day04b_sample() {
        let pairs = parse_input_a(RAW_INPUT);
        let partial_overlaps = process_input_b(&pairs);
        assert_eq!(partial_overlaps, 4)
    }
}
