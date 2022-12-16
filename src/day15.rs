use std::{collections::HashSet, fs, str::FromStr};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self},
    combinator::map,
    error::Error,
    sequence::{preceded, separated_pair},
    Finish, IResult,
};

use crate::nomstr;

pub fn day15a() -> String {
    let data = fs::read_to_string("assets/day15.txt").expect("Could not load file");
    let sensors = parse_input_a(&data);
    let count = process_input_a(&sensors, 2000000);
    count.to_string()
}

pub fn day15b() -> String {
    let data = fs::read_to_string("assets/day15.txt").expect("Could not load file");
    data
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Coordinate {
    x: i64,
    y: i64,
}

impl Coordinate {
    pub fn parse(i: &str) -> IResult<&str, Coordinate> {
        map(
            separated_pair(
                preceded(tag("x="), complete::i64),
                tag(", "),
                preceded(tag("y="), complete::i64),
            ),
            |(x, y)| Coordinate { x, y },
        )(i)
    }
}

nomstr!(Coordinate);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Sensor {
    position: Coordinate,
    beacon: Coordinate,
}

impl Sensor {
    pub fn parse(i: &str) -> IResult<&str, Sensor> {
        let (i, position) = preceded(tag("Sensor at "), Coordinate::parse)(i)?;
        let (i, beacon) = preceded(tag(": closest beacon is at "), Coordinate::parse)(i)?;
        Ok((i, Sensor { position, beacon }))
    }

    // Computes the manhattan distance between the sensor and its beacon.
    //
    // https://en.wikipedia.org/wiki/Taxicab_geometry
    pub fn manhattan_distance(&self) -> i64 {
        (self.beacon.x - self.position.x).abs() + (self.beacon.y - self.position.y).abs()
    }

    pub fn no_beacon_zone(&self) -> HashSet<Coordinate> {
        let mut no_beacon_zone = HashSet::new();
        let distance = self.manhattan_distance();

        for y_ in -distance..=distance {
            let y = self.position.y + y_;
            let x_boundary = (y_.abs() - distance).abs();
            for x in (self.position.x - x_boundary)..=(self.position.x + x_boundary) {
                no_beacon_zone.insert(Coordinate { x, y });
            }
        }

        // Remove self.
        no_beacon_zone.remove(&self.beacon);

        // Return the no beacon zone.
        no_beacon_zone
    }

    pub fn beacon_tunning_frequency(&self, boundary: i64) -> i64 {
        self.beacon.x * boundary + self.beacon.y
    }
}

nomstr!(Sensor);

pub fn parse_input_a(i: &str) -> Vec<Sensor> {
    i.lines()
        .map(|l| l.parse::<Sensor>().unwrap())
        .collect_vec()
}

pub fn process_input_a(sensors: &[Sensor], row: i64) -> usize {
    let mut no_beacon_zone = HashSet::new();
    for sensor in sensors {
        let distance = sensor.manhattan_distance();
        // There is no need for the useless y_ loop here!
        //
        // for y_ in -distance..=distance {
        //     let y = sensor.position.y + y_;
        //     if y == row {
        //         let x_boundary = (y_.abs() - distance).abs();
        //         for x in (sensor.position.x - x_boundary)..=(sensor.position.x + x_boundary) {
        //             no_beacon_zone.insert(Coordinate { x, y });
        //         }
        //     }
        // }
        let y_ = row - sensor.position.y;
        let x_boundary = (y_.abs() - distance).abs();
        for x in (sensor.position.x - x_boundary)..=(sensor.position.x + x_boundary) {
            no_beacon_zone.insert(Coordinate { x, y: row });
        }
    }

    for sensor in sensors {
        no_beacon_zone.remove(&sensor.beacon);
    }

    no_beacon_zone.len()
}

pub fn process_input_b(sensors: &[Sensor], boundary: i64) -> usize {
    // let tunning_frequency = sensor.beacon.x * boundary + sensor.beacon.y;
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const RAW_INPUT: &'static str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

    #[test]
    fn test_day15a_sample() {
        let sensors = parse_input_a(RAW_INPUT);
        let count = process_input_a(&sensors, 10);
        assert_eq!(count, 26);
    }

    #[test]
    fn test_day15b_sample() {}

    #[test]
    fn test_sensor_parse() {
        let sensor = "Sensor at x=8, y=7: closest beacon is at x=2, y=10"
            .parse::<Sensor>()
            .unwrap();
        assert_eq!(
            sensor,
            Sensor {
                position: Coordinate { x: 8, y: 7 },
                beacon: Coordinate { x: 2, y: 10 }
            }
        );
        assert_eq!(sensor.manhattan_distance(), 9);
        assert_eq!(sensor.no_beacon_zone().len(), 180);
    }
}
