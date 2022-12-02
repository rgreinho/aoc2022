use std::{cmp::Ordering, fs};

pub fn day02a() -> String {
    let data = fs::read_to_string("assets/day02.txt").expect("Could not load file");
    let battles = parse_input(&data);
    process_input(&battles).to_string()
}

pub fn parse_input(input: &str) -> Vec<Battle> {
    input.lines().map(String::from).map(Battle::from).collect()
}

pub fn process_input(battles: &[Battle]) -> u32 {
    battles.iter().map(|battle| battle.score()).sum()
}

pub fn day02b() -> String {
    let data = fs::read_to_string("assets/day02.txt").expect("Could not load file");
    let battles = parse_input_2(&data);
    process_input_2(&battles).to_string()
}

pub fn parse_input_2(input: &str) -> Vec<BattleTwo> {
    input
        .lines()
        .map(String::from)
        .map(BattleTwo::from)
        .collect()
}

pub fn process_input_2(battles: &[BattleTwo]) -> u32 {
    dbg!(&battles);
    battles
        .iter()
        .map(|battle| battle.our_play() as u32 + battle.outcome as u32)
        .sum()
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl From<char> for Shape {
    fn from(item: char) -> Self {
        match item {
            'A' => Shape::Rock,
            'B' => Shape::Paper,
            'C' => Shape::Scissors,
            'X' => Shape::Rock,
            'Y' => Shape::Paper,
            'Z' => Shape::Scissors,
            _ => panic!("Cannot parse shape '{}'.", item),
        }
    }
}

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let cmp = match self {
            Shape::Rock => match other {
                Shape::Rock => Ordering::Equal,
                Shape::Paper => Ordering::Less,
                Shape::Scissors => Ordering::Greater,
            },
            Shape::Paper => match other {
                Shape::Rock => Ordering::Greater,
                Shape::Paper => Ordering::Equal,
                Shape::Scissors => Ordering::Less,
            },
            Shape::Scissors => match other {
                Shape::Rock => Ordering::Less,
                Shape::Paper => Ordering::Greater,
                Shape::Scissors => Ordering::Equal,
            },
        };
        Some(cmp)
    }

    fn lt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Ordering::Less))
    }

    fn le(&self, other: &Self) -> bool {
        matches!(
            self.partial_cmp(other),
            Some(Ordering::Less | Ordering::Equal)
        )
    }

    fn gt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Ordering::Greater))
    }

    fn ge(&self, other: &Self) -> bool {
        matches!(
            self.partial_cmp(other),
            Some(Ordering::Greater | Ordering::Equal)
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl From<char> for Outcome {
    fn from(item: char) -> Self {
        match item {
            'X' => Outcome::Loss,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => panic!("Cannot parse shape '{}'.", item),
        }
    }
}

#[derive(Debug)]
pub struct Battle {
    theirs: Shape,
    ours: Shape,
}

impl From<String> for Battle {
    fn from(item: String) -> Self {
        if item.len() != 3 {
            panic!("Invalid entry: {}.", item)
        }
        let mut chars = item.chars();
        let theirs = Shape::from(chars.next().unwrap());
        let ours = Shape::from(chars.nth(1).unwrap());
        Self { theirs, ours }
    }
}

impl Battle {
    pub fn score(&self) -> u32 {
        if self.theirs > self.ours {
            return Outcome::Loss as u32 + self.ours as u32;
        } else if self.theirs < self.ours {
            return Outcome::Win as u32 + self.ours as u32;
        } else {
            Outcome::Draw as u32 + self.ours as u32
        }
    }
}

#[derive(Debug)]
pub struct BattleTwo {
    theirs: Shape,
    outcome: Outcome,
}

impl From<String> for BattleTwo {
    fn from(item: String) -> Self {
        if item.len() != 3 {
            panic!("Invalid entry: {}.", item)
        }
        let mut chars = item.chars();
        let theirs = Shape::from(chars.next().unwrap());
        let outcome = Outcome::from(chars.nth(1).unwrap());
        Self { theirs, outcome }
    }
}

impl BattleTwo {
    fn our_play(&self) -> Shape {
        match self.outcome {
            Outcome::Loss => match self.theirs {
                Shape::Rock => Shape::Scissors,
                Shape::Paper => Shape::Rock,
                Shape::Scissors => Shape::Paper,
            },
            Outcome::Draw => self.theirs,
            Outcome::Win => match self.theirs {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissors,
                Shape::Scissors => Shape::Rock,
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const RAW_INPUT: &'static str = "A Y
B X
C Z";

    #[test]
    fn test_day02_parta_sample() {
        let input = parse_input(RAW_INPUT);
        let res = process_input(&input);
        assert_eq!(res, 15)
    }

    #[test]
    fn test_day02_partb_sample() {
        let input = parse_input_2(RAW_INPUT);
        let res = process_input_2(&input);
        assert_eq!(res, 12)
    }
}
