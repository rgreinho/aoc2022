use std::{collections::HashSet, fs};

pub fn day03a() -> String {
    let data = fs::read_to_string("assets/day03.txt").expect("Could not load file");
    let rucksacks = parse_input(&data);
    let priorities = process_input_a(&rucksacks);
    priorities.to_string()
}

pub fn day03b() -> String {
    let data = fs::read_to_string("assets/day03.txt").expect("Could not load file");
    let rucksacks = parse_input(&data);
    let priorities = process_input_b(&rucksacks, 3);
    priorities.to_string()
}

#[derive(Debug)]
pub struct Rucksack {
    left: Vec<char>,
    right: Vec<char>,
    whole: String,
}

impl From<&str> for Rucksack {
    fn from(item: &str) -> Self {
        if item.len() % 2 != 0 {
            panic!("Invalid length for item {item}: {}", item.len());
        }
        let mid = item.len() / 2;
        let (left, right) = item.split_at(mid);
        Rucksack {
            left: left.chars().collect(),
            right: right.chars().collect(),
            whole: String::from(item),
        }
    }
}

impl Rucksack {
    pub fn common_item(&self) -> char {
        let left: HashSet<char> = self.left.iter().map(|&c| c).collect();
        let right: HashSet<char> = self.right.iter().map(|&c| c).collect();
        let intersection: Vec<char> = left.intersection(&right).map(|&c| c).collect();
        if intersection.len() != 1 {
            panic!("Too many common items in the rucksack: {:?}", intersection)
        }
        *intersection.first().unwrap()
    }

    pub fn to_hashset(&self) -> HashSet<char> {
        self.whole.chars().collect()
    }
}

// To help prioritize item rearrangement, every item type can be converted to a priority:
//     Lowercase item types a through z have priorities 1 through 26.
//     Uppercase item types A through Z have priorities 27 through 52.
pub fn priority(item: char) -> u32 {
    // [src/main.rs:2] 'a' as u8 = 97
    // [src/main.rs:3] 'z' as u8 = 122
    // [src/main.rs:4] 'A' as u8 = 65
    // [src/main.rs:5] 'Z' as u8 = 90
    let value = item as u32;
    if (97..=122).contains(&value) {
        return value - 96;
    }
    if (65..=90).contains(&value) {
        return value - 38;
    }

    panic!("Invalid item: {item}/{value}")
}

pub fn parse_input(input: &str) -> Vec<Rucksack> {
    input.lines().map(Rucksack::from).collect()
}

pub fn process_input_a(rucksacks: &[Rucksack]) -> u32 {
    rucksacks
        .iter()
        .map(|r| r.common_item())
        .map(|i| priority(i))
        .sum()
}

pub fn process_input_b(rucksacks: &[Rucksack], size: usize) -> u32 {
    let mut badge_sum = 0;

    // Split the rucksacks into groups.
    let groups = rucksacks.chunks(size);
    for group in groups {
        let mut iter = group.iter();

        // Check the first rucksack.
        let mut group_badge = HashSet::from(iter.next().unwrap().to_hashset());

        // Look for the common items with the other ricksacks of the group.
        for next in iter {
            let other = next.to_hashset();
            let intersection: HashSet<_> = group_badge.intersection(&other).collect();
            group_badge = intersection.iter().map(|&&c| c).collect();
        }

        // Ensure there is a single badge at the end of the process.
        if group_badge.len() != 1 {
            panic!("Too many badges in the group: {:?}", group_badge)
        }

        // Add its priority value to the total.
        let group_badge_vec = group_badge.iter().map(|&c| c).collect::<Vec<char>>();
        badge_sum += priority(*group_badge_vec.first().unwrap());
    }
    badge_sum
}

#[cfg(test)]
mod test {
    use super::*;

    const RAW_INPUT: &'static str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

    #[test]
    fn test_day03a_sample() {
        let rucksacks = parse_input(RAW_INPUT);
        let priorities = process_input_a(&rucksacks);
        assert_eq!(priorities, 157)
    }

    #[test]
    fn test_day03b_sample() {
        let rucksacks = parse_input(RAW_INPUT);
        let priorities = process_input_b(&rucksacks, 3);
        assert_eq!(priorities, 70)
    }
}
