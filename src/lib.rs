use std::fs;
use std::path::Path;
use std::str::FromStr;

// pub mod day01;

pub fn read_input<P, T>(input: P) -> Vec<T>
where
    P: AsRef<Path>,
    T: FromStr,
{
    let values = fs::read_to_string(input).expect("Could not load file");
    input_from_string::<T>(&values)
}

pub fn input_from_string<T>(input: &str) -> Vec<T>
where
    T: FromStr,
{
    input
        .lines()
        .filter_map(|s| s.parse::<T>().ok())
        .collect::<Vec<T>>()
}

pub fn read_input_sep<P, T>(input: P, separator: &str) -> Vec<T>
where
    P: AsRef<Path>,
    T: FromStr,
{
    let values = fs::read_to_string(input).expect("Could not load file");
    input_sep_from_string::<T>(&values, separator)
}

pub fn input_sep_from_string<T>(input: &str, separator: &str) -> Vec<T>
where
    T: FromStr,
{
    input
        .trim()
        .split(separator)
        .filter_map(|s| s.parse::<T>().ok())
        .collect::<Vec<T>>()
}
