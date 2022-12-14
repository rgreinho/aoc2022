use std::{
    collections::{HashMap, HashSet},
    fs,
    str::FromStr,
};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::map,
    error::Error,
    multi::separated_list1,
    sequence::separated_pair,
    Finish, IResult,
};

pub fn day14a() -> String {
    let data = fs::read_to_string("assets/day14.txt").expect("Could not load file");
    let mut cave = data.parse::<Cave>().unwrap();
    let resting_sand = process_input_a(&mut cave);
    resting_sand.to_string()
}

pub fn day14b() -> String {
    let data = fs::read_to_string("assets/day14.txt").expect("Could not load file");
    let mut cave = data.parse::<Cave>().unwrap();
    let resting_sand = process_input_b(&mut cave);
    resting_sand.to_string()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Coordinate {
    x: u32,
    y: u32,
}

impl Coordinate {
    pub fn parse(i: &str) -> IResult<&str, Coordinate> {
        map(
            separated_pair(complete::u32, complete::char(','), complete::u32),
            |(x, y)| Coordinate { x, y },
        )(i)
    }

    pub fn down(&mut self) {
        self.y += 1;
    }

    pub fn down_left(&mut self) {
        self.x -= 1;
        self.y += 1;
    }

    pub fn down_right(&mut self) {
        self.x += 1;
        self.y += 1;
    }
}

impl FromStr for Coordinate {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Coordinate::parse(s).finish() {
            Ok((_, item)) => Ok(item),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Wall(HashSet<Coordinate>);

impl Wall {
    pub fn parse(i: &str) -> IResult<&str, Wall> {
        let (i, coordinates) = separated_list1(tag(" -> "), Coordinate::parse)(i)?;
        let rocks = coordinates
            .iter()
            .tuple_windows()
            .flat_map(|(c0, c1)| {
                let x_min = c0.x.min(c1.x);
                let x_max = c0.x.max(c1.x);
                let x_range = x_min..=x_max;

                let y_min = c0.y.min(c1.y);
                let y_max = c0.y.max(c1.y);
                let y_range = y_min..=y_max;

                x_range
                    .cartesian_product(y_range)
                    .map(|(x, y)| Coordinate { x, y })
            })
            .collect::<HashSet<Coordinate>>();
        Ok((i, Wall(rocks)))
    }

    pub fn add(&mut self, rock: &Coordinate) {
        self.0.insert(rock.clone());
    }
}

impl FromStr for Wall {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Wall::parse(s).finish() {
            Ok((_, item)) => Ok(item),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

#[derive(Debug)]
pub enum Tile {
    Rock,
    Sand,
}

#[derive(Debug)]
pub struct Cave(HashMap<Coordinate, Tile>);

impl Cave {
    pub fn parse(i: &str) -> IResult<&str, Cave> {
        let (i, walls) = separated_list1(line_ending, Wall::parse)(i)?;
        let mut cave = Cave(HashMap::new());
        for wall in walls {
            for coordinate in wall.0 {
                cave.0.insert(coordinate, Tile::Rock);
            }
        }
        Ok((i, cave))
    }

    pub fn lowest_point(&self) -> Coordinate {
        let mut coordinates = self.0.keys().collect_vec();
        coordinates.sort_by(|c1, c2| c1.y.cmp(&c2.y));
        **coordinates.last().unwrap()
    }

    pub fn add(&mut self, coordinate: Coordinate, tile: Tile) {
        self.0.insert(coordinate, tile);
    }
}

impl FromStr for Cave {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Cave::parse(s).finish() {
            Ok((_, item)) => Ok(item),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

pub fn process_input_a(cave: &mut Cave) -> u32 {
    let lowest_point = cave.lowest_point();
    let mut resting_sand = 0;
    let mut current_sand = Coordinate { x: 500, y: 0 };
    while current_sand.y < lowest_point.y {
        let (d, dl, dr) = get_next_row(&cave, current_sand);
        // Try to go down.
        if d.is_none() {
            current_sand.down();
            continue;
        }

        // Try to go down-left.
        if dl.is_none() {
            current_sand.down_left();
            continue;
        }

        // Try to go down-right.
        if dr.is_none() {
            current_sand.down_right();
            continue;
        }

        // If frozen, reset the current sand.
        cave.add(current_sand.clone(), Tile::Sand);
        resting_sand += 1;
        current_sand = Coordinate { x: 500, y: 0 };
    }

    resting_sand
}

pub fn process_input_b(cave: &mut Cave) -> u32 {
    let mut lowest_point = cave.lowest_point();
    lowest_point.y += 2;
    let mut resting_sand = 0;
    let mut current_sand = Coordinate { x: 500, y: 0 };

    while cave.0.get(&Coordinate { x: 500, y: 0 }).is_none() {
        if current_sand.y == lowest_point.y - 1 {
            cave.add(current_sand.clone(), Tile::Sand);
            resting_sand += 1;
            current_sand = Coordinate { x: 500, y: 0 };
        }
        let (d, dl, dr) = get_next_row(&cave, current_sand);
        // Try to go down.
        if d.is_none() {
            current_sand.down();
            continue;
        }

        // Try to go down-left.
        if dl.is_none() {
            current_sand.down_left();
            continue;
        }

        // Try to go down-right.
        if dr.is_none() {
            current_sand.down_right();
            continue;
        }

        // If frozen, reset the current sand.
        cave.add(current_sand.clone(), Tile::Sand);
        resting_sand += 1;
        current_sand = Coordinate { x: 500, y: 0 };
    }

    resting_sand
}

pub fn get_next_row(cave: &Cave, c: Coordinate) -> (Option<&Tile>, Option<&Tile>, Option<&Tile>) {
    let d = cave.0.get(&Coordinate { x: c.x, y: c.y + 1 });
    let dl = cave.0.get(&Coordinate {
        x: c.x - 1,
        y: c.y + 1,
    });
    let dr = cave.0.get(&Coordinate {
        x: c.x + 1,
        y: c.y + 1,
    });
    (d, dl, dr)
}

#[cfg(test)]
mod test {
    use super::*;

    const RAW_INPUT: &'static str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";

    #[test]
    fn test_day14a_sample() {
        let mut cave = RAW_INPUT.parse::<Cave>().unwrap();
        let resting_sand = process_input_a(&mut cave);
        assert_eq!(resting_sand, 24);
    }

    #[test]
    fn test_day14b_sample() {
        let mut cave = RAW_INPUT.parse::<Cave>().unwrap();
        let resting_sand = process_input_b(&mut cave);
        assert_eq!(resting_sand, 93);
    }

    #[test]
    fn test_parse_wall() {
        let wall = "498,4 -> 498,6 -> 496,6".parse::<Wall>().unwrap();
        let mut expected_wall = Wall(HashSet::new());
        expected_wall.add(&Coordinate { x: 498, y: 4 });
        expected_wall.add(&Coordinate { x: 498, y: 5 });
        expected_wall.add(&Coordinate { x: 498, y: 6 });
        expected_wall.add(&Coordinate { x: 496, y: 6 });
        expected_wall.add(&Coordinate { x: 497, y: 6 });
        expected_wall.add(&Coordinate { x: 498, y: 6 });
        assert_eq!(wall, expected_wall);
        assert_eq!(wall.0.len(), 5)
    }
}
