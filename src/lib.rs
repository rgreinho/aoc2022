use std::{fs, path::Path, str::FromStr};

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;

pub fn read_input<P, T>(input: P) -> Vec<T>
where
    P: AsRef<Path>,
    T: FromStr,
{
    let values = fs::read_to_string(input).expect("Could not load file");
    input_from_string::<T>(&values)
}
/// Read the content of a string and parse each line into a Vector.
///
/// ```
/// use aoc2022::input_from_string;
///
/// assert_eq!(input_from_string::<u8>("1\n6\n8\n32\n112\n"), vec![1,6,8,32,112]);
/// ```
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

/// Read the content of a string, split based on the separator and parse each line into a Vector.
///
/// ```
/// use aoc2022::input_sep_from_string;
///
/// assert_eq!(input_sep_from_string::<u8>("1,6,8,32,112\n", ","), vec![1,6,8,32,112]);
/// ```
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

// Create a FromStr implementation for a specific type.
//
// This macros expects the following items to be setup before use:
//   - `use std::str::FromStr` must be in scope
//   - the type must implement a `parse` function with the following signature
//       `pub fn parse(i: &str) -> IResult<&str, __type__>`
#[macro_export]
macro_rules! nomstr {
    ($a:ident) => {
        impl FromStr for $a {
            type Err = Error<String>;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match $a::parse(s).finish() {
                    Ok((_, item)) => Ok(item),
                    Err(Error { input, code }) => Err(Error {
                        input: input.to_string(),
                        code,
                    }),
                }
            }
        }
    };
}
