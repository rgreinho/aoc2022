use std::{collections::HashSet, fs};

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

impl From<&str> for Assignement {
    fn from(item: &str) -> Self {
        let v: Vec<&str> = item.split('-').collect();
        if v.len() != 2 {
            panic!("Invalid entry: {item}.")
        }
        let start = v.first().unwrap().parse::<u32>().unwrap();
        let end = v.last().unwrap().parse::<u32>().unwrap();
        let section: HashSet<u32> = (start..=end).collect();
        Assignement { section }
    }
}

#[derive(Debug)]
pub struct Pair(Assignement, Assignement);

impl From<&str> for Pair {
    fn from(item: &str) -> Self {
        let v: Vec<&str> = item.split(',').collect();
        if v.len() != 2 {
            panic!("Invalid entry: {item}.")
        }
        let s1: Assignement = v.first().map(|&e| Assignement::from(e)).unwrap();
        let s2: Assignement = v.last().map(|&e| Assignement::from(e)).unwrap();
        Pair(s1, s2)
    }
}

impl Pair {
    pub fn overlap(&self) -> bool {
        self.0.section.is_superset(&self.1.section) || self.1.section.is_superset(&self.0.section)
    }

    pub fn partial_overlap(&self) -> bool {
        let intersection: HashSet<_> = self.0.section.intersection(&self.1.section).collect();
        !intersection.is_empty()
    }
}

pub fn parse_input_a(input: &str) -> Vec<Pair> {
    input.lines().map(Pair::from).collect()
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
