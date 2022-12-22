use std::fs;

use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::{self, line_ending},
    combinator::{eof, iterator},
    multi::separated_list1,
    sequence::terminated,
    IResult,
};

pub fn day20a() -> String {
    let data = fs::read_to_string("assets/day20.txt").expect("Could not load file");
    let (_, encrypted_file) = parse_input(&data).unwrap();
    process_input_a(&encrypted_file, 1).to_string()
}

pub fn day20b() -> String {
    let data = fs::read_to_string("assets/day20.txt").expect("Could not load file");
    let (_, mut encrypted_file) = parse_input(&data).unwrap();
    prep_input_b(&mut encrypted_file);
    process_input_a(&encrypted_file, 10).to_string()
}

pub fn parse_input(i: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(complete::line_ending, complete::i64)(i)
}

pub fn process_input_a(encrypted_file: &[i64], mix: u8) -> i64 {
    let id_map = encrypted_file
        .iter()
        .enumerate()
        .map(|(i, &v)| (i, v))
        .collect::<Vec<(usize, i64)>>();
    let mut state = id_map.clone();

    // Mix the map.
    for _ in 0..mix {
        for index in 0..encrypted_file.len() {
            let position = state.iter().position(|(i, _v)| index == *i).unwrap();
            let (i, v) = state.remove(position);
            // The error here was using encrypted_file.len() instead of state.len().
            // since state is shorter by one at that time, it makes sense to use
            // this new length before inserting the value.
            let new_pos = (position as i64 + v).rem_euclid(state.len() as i64) as usize;
            state.insert(new_pos, (i, v));
        }
    }

    // Decrypt.
    let zero_pos = state.iter().position(|(_i, v)| *v == 0).unwrap();
    (1..=3)
        .map(|i| state[(i * 1000 + zero_pos) % state.len()].1)
        .sum()
}

pub fn prep_input_b(encrypted_file: &mut [i64]) {
    const DECRYPTION_KEY: i64 = 811589153;
    for v in encrypted_file {
        *v = *v * DECRYPTION_KEY;
    }
}

// Solution from Chris Bacardi to help me debug my solution.
// https://github.com/ChristopherBiscardi/advent-of-code/blob/main/2022/rust/day-20/src/lib.rs
fn numbers(input: &str) -> IResult<&str, Vec<(usize, i64)>> {
    let mut it = iterator(input, terminated(complete::i64, alt((line_ending, eof))));
    let numbers = it.enumerate().collect::<Vec<_>>();
    let (input, _) = it.finish()?;
    Ok((input, numbers))
}

pub fn process_part1(input: &str) -> i64 {
    let (_, numbers) = numbers(input).unwrap();
    let mut state = numbers.clone();
    for (id, _value) in numbers.iter() {
        let index = state
            .iter()
            .position(|state_value| state_value.0 == *id)
            .unwrap();

        let current = state.remove(index);
        let added = index as i64 + current.1;
        let new_index = added.rem_euclid(state.len() as i64);

        state.insert(new_index as usize, current);
    }

    let zero_pos = state.iter().position(|v| v.1 == 0).unwrap();
    let a = state[(1000 + zero_pos) % state.len()].1;
    let b = state[(2000 + zero_pos) % state.len()].1;
    let c = state[(3000 + zero_pos) % state.len()].1;
    a + b + c
}

#[cfg(test)]
mod test {
    use super::*;

    const RAW_INPUT: &'static str = "1
2
-3
3
-2
0
4
";

    #[test]
    fn test_day20a_sample() {
        let (_, encrypted_file) = parse_input(RAW_INPUT).unwrap();
        assert_eq!(process_input_a(&encrypted_file, 1), 3);
    }

    #[test]
    fn test_day20a_chris() {
        assert_eq!(process_part1(RAW_INPUT), 3);
    }

    #[test]
    fn test_day20b_sample() {
        let (_, mut encrypted_file) = parse_input(RAW_INPUT).unwrap();
        prep_input_b(&mut encrypted_file);
        assert_eq!(process_input_a(&encrypted_file, 10), 1623178306);
    }
}
