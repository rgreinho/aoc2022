use aoc2022::day01::{day01a, day01b};
use aoc2022::day02::{day02a, day02b};
use aoc2022::day03::{day03a, day03b};
use aoc2022::day04::{day04a, day04b};
use aoc2022::day05::{day05a, day05b};
use aoc2022::day06::{day06a, day06b};
use aoc2022::day07::{day07a, day07b};
use aoc2022::day08::{day08a, day08b};
use aoc2022::day09::{day09a, day09b};
use aoc2022::day10::{day10a, day10b};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let problem = args.get(1).map(|s| s.as_str()).unwrap_or("None");
    let result = match problem {
        "day01a" => day01a(),
        "day01b" => day01b(),
        "day02a" => day02a(),
        "day02b" => day02b(),
        "day03a" => day03a(),
        "day03b" => day03b(),
        "day04a" => day04a(),
        "day04b" => day04b(),
        "day05a" => day05a(),
        "day05b" => day05b(),
        "day06a" => day06a(),
        "day06b" => day06b(),
        "day07a" => day07a(),
        "day07b" => day07b(),
        "day08a" => day08a(),
        "day08b" => day08b(),
        "day09a" => day09a(),
        "day09b" => day09b(),
        "day10a" => day10a(),
        "day10b" => day10b(),
        _ => "We haven't solved that yet".to_string(),
    };
    println!("{}", result);
}
