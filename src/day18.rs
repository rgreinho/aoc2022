use crate::nomstr;
use nom::{
    bytes::complete::tag, character::complete, combinator::map, error::Error,
    multi::separated_list1, Finish, IResult,
};
use std::{collections::HashSet, fs, ops::Add, str::FromStr};

pub fn day18a() -> String {
    let data = fs::read_to_string("assets/day18.txt").expect("Could not load file");
    let cubes = parse_input_a(&data);
    let not_connected = process_input_a(&cubes);
    not_connected.to_string()
}

pub fn day18b() -> String {
    let data = fs::read_to_string("assets/day18.txt").expect("Could not load file");
    data
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct Coordinate {
    x: i32,
    y: i32,
    z: i32,
}

impl Add for Coordinate {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.y + rhs.z,
        }
    }
}

impl Coordinate {
    pub fn parse(i: &str) -> IResult<&str, Coordinate> {
        map(separated_list1(tag(","), complete::i32), |v| Coordinate {
            x: v[0],
            y: v[1],
            z: v[2],
        })(i)
    }

    pub fn neighbors(&self) -> Vec<Coordinate> {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        vec![
            Coordinate {
                x: self.x - 1,
                y,
                z,
            },
            Coordinate {
                x: self.x + 1,
                y,
                z,
            },
            Coordinate {
                x,
                y: self.y - 1,
                z,
            },
            Coordinate {
                x,
                y: self.y + 1,
                z,
            },
            Coordinate {
                x,
                y,
                z: self.z - 1,
            },
            Coordinate {
                x,
                y,
                z: self.z + 1,
            },
        ]
    }
}
nomstr!(Coordinate);

pub fn parse_input_a(i: &str) -> Vec<Coordinate> {
    i.lines()
        .map(|c| c.parse::<Coordinate>().unwrap())
        .collect()
}

pub fn process_input_a(coordinates: &[Coordinate]) -> usize {
    let cubes: HashSet<Coordinate> = HashSet::from_iter(coordinates.iter().map(|&c| c));
    cubes
        .iter()
        .map(|&cube| {
            // Count the sides without neighbor.
            cube.neighbors()
                .iter()
                .filter(|c| cubes.get(&c).is_none())
                .count()
        })
        .sum::<usize>()
}

#[cfg(test)]
mod test {
    use super::*;

    const RAW_INPUT: &'static str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";

    #[test]
    fn test_day18a_sample() {
        let cubes = parse_input_a(RAW_INPUT);
        let not_connected = process_input_a(&cubes);
        assert_eq!(not_connected, 64);
    }

    #[test]
    fn test_day18b_sample() {}
}
