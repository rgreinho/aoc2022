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
use aoc2022::day11::{day11a, day11b};
use aoc2022::day12::{day12a, day12b};
use aoc2022::day13::{day13a, day13b};
use aoc2022::day14::{day14a, day14b};
use aoc2022::day15::{day15a, day15b};
use aoc2022::day16::{day16a, day16b};
use aoc2022::day17::{day17a, day17b};
use aoc2022::day18::{day18a, day18b};

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
        "day11a" => day11a(),
        "day11b" => day11b(),
        "day12a" => day12a(),
        "day12b" => day12b(),
        "day13a" => day13a(),
        "day13b" => day13b(),
        "day14a" => day14a(),
        "day14b" => day14b(),
        "day15a" => day15a(),
        "day15b" => day15b(),
        "day16a" => day16a(),
        "day16b" => day16b(),
        "day17a" => day17a(),
        "day17b" => day17b(),
        "day18a" => day18a(),
        "day18b" => day18b(),
        _ => "We haven't solved that yet".to_string(),
    };
    println!("{}", result);
}
