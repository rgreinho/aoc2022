use crate::input_sep_from_string;
use std::fs;

pub fn day01a() -> String {
    let max = process_a(&get_groups());
    max.to_string()
}

pub fn day01b() -> String {
    let top_three = process_b(&get_groups());
    top_three.to_string()
}

pub fn parse_input(input: &str) -> Vec<String> {
    input_sep_from_string::<String>(input, "\n\n")
}

fn group_sums(input: &[String]) -> Vec<u32> {
    input
        .iter()
        .map(|l| l.lines().filter_map(|s| s.parse::<u32>().ok()).sum())
        .collect()
}

fn get_groups() -> Vec<u32> {
    let data = fs::read_to_string("assets/day01.txt").expect("Could not load file");
    let input = parse_input(&data);
    group_sums(&input)
}

pub fn process_a(input: &[u32]) -> u32 {
    *input.iter().max().unwrap()
}

pub fn process_b(input: &[u32]) -> u32 {
    let mut cloned_input = input.iter().copied().collect::<Vec<u32>>();
    cloned_input.sort();
    cloned_input.into_iter().rev().take(3).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const RAW_INPUT: &'static str = "1000
2000
3000

400

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_day01_parta_sample() {
        let input = parse_input(RAW_INPUT);
        let groups = group_sums(&input);
        let res = process_a(&groups);
        assert_eq!(res, 24000);
    }

    #[test]
    fn test_day02_partb_sample() {
        let input = parse_input(RAW_INPUT);
        let groups = group_sums(&input);
        let res = process_b(&groups);
        assert_eq!(res, 45000);
    }
}
