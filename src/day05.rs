use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

pub fn day05a() -> String {
    let data = fs::read_to_string("assets/day05.txt").expect("Could not load file");
    let (mut stacks, moves) = parse_input_a(&data);
    process_input_a(&mut stacks, &moves)
}

pub fn day05b() -> String {
    let data = fs::read_to_string("assets/day05.txt").expect("Could not load file");
    let (mut stacks, moves) = parse_input_a(&data);
    process_input_b(&mut stacks, &moves)
}

#[derive(Debug, Clone, Copy)]
pub struct Crate(char);

impl From<&str> for Crate {
    fn from(item: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\[([[:upper:]])\]\s?").unwrap();
        }
        if let Some(caps) = RE.captures(item) {
            return Crate(caps.get(1).unwrap().as_str().chars().nth(0).unwrap());
        }
        panic!("Cannot parse move: `{item}`.")
    }
}

// #[derive(Debug)]
// pub struct Stack(Vec<Crate>);

// #[derive(Debug)]
// pub struct Supplies(Vec<Stack>);

#[derive(Debug)]
pub struct Move {
    count: u32,
    from: usize,
    to: usize,
}

// move 8 from 7 to 1
impl From<&str> for Move {
    fn from(item: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r#"(?x)
        move
        \s
        (?P<move>\d+)
        \s
        from
        \s
        (?P<from>\d+)
        \s
        to
        \s
        (?P<to>\d+)
        "#,
            )
            .unwrap();
        }
        if let Some(caps) = RE.captures(item) {
            let count = caps.name("move").unwrap().as_str().parse::<u32>().unwrap();
            let from = caps
                .name("from")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap()
                - 1;
            let to = caps.name("to").unwrap().as_str().parse::<usize>().unwrap() - 1;
            return Move { count, from, to };
        }
        panic!("Cannot parse move: `{item}`.")
    }
}

pub fn parse_input_a(input: &str) -> (Vec<Vec<Crate>>, Vec<Move>) {
    let split_input: Vec<&str> = input.split("\n\n").collect();
    let raw_stacks = split_input.first().unwrap();
    let stacks = parse_stacks(raw_stacks);
    let raw_moves = split_input.last().unwrap();
    let moves = parse_moves(raw_moves);
    (stacks, moves)
}

pub fn process_input_a(stacks: &mut Vec<Vec<Crate>>, moves: &[Move]) -> String {
    for m in moves {
        for _ in 0..m.count {
            let crate_ = stacks[m.from].pop().unwrap();
            stacks[m.to].push(crate_);
        }
    }
    message(&stacks)
}

pub fn process_input_b(stacks: &mut Vec<Vec<Crate>>, moves: &[Move]) -> String {
    for m in moves {
        let mut chunks: Vec<Crate> = Vec::new();
        for _ in 0..m.count {
            let crate_ = stacks[m.from].pop().unwrap();
            chunks.insert(0, crate_);
        }
        stacks[m.to].extend_from_slice(&chunks);
    }
    message(&stacks)
}

pub fn parse_stacks(input: &str) -> Vec<Vec<Crate>> {
    let raw_stacks = input
        .lines()
        .rev()
        .skip(1)
        .map(String::from)
        .collect::<Vec<String>>();
    let stack_count = raw_stacks.first().unwrap().split_whitespace().count();
    let mut stacks: Vec<Vec<Crate>> = vec![vec![]; stack_count];
    for stack_line in raw_stacks {
        let line = stack_line.chars().collect::<Vec<char>>();
        for (i, crate_) in line.chunks(4).enumerate() {
            let c: String = crate_.iter().collect();
            if !c.trim().is_empty() {
                stacks[i].push(Crate::from(c.as_str()));
            }
        }
    }
    stacks
}

pub fn parse_moves(input: &str) -> Vec<Move> {
    input.lines().map(Move::from).collect::<Vec<Move>>()
}

pub fn message(stacks: &Vec<Vec<Crate>>) -> String {
    let mut message = String::new();
    for stack in stacks {
        message.push(stack.last().unwrap().0);
    }
    message
}

#[cfg(test)]
mod test {
    use super::*;

    const RAW_INPUT: &'static str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn test_day05a_sample() {
        let (mut stacks, moves) = parse_input_a(RAW_INPUT);
        let message = process_input_a(&mut stacks, &moves);
        assert_eq!(message, String::from("CMZ"));
    }

    #[test]
    fn test_day05b_sample() {
        let (mut stacks, moves) = parse_input_a(RAW_INPUT);
        let message = process_input_b(&mut stacks, &moves);
        assert_eq!(message, String::from("MCD"));
    }
}
