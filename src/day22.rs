use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::value,
    multi::{many1, separated_list1},
    IResult, Parser,
};
use std::{fmt::Display, fs};

pub fn day22a() -> String {
    let data = fs::read_to_string("assets/day22.txt").expect("Could not load file");
    let (i, jungle) = Jungle::parse(&data).unwrap();
    let (_, motions) = parse_motions(i.trim()).unwrap();
    let password = process_input_a(jungle, &motions);
    password.to_string()
    // 33242 is too low
}

pub fn day22b() -> String {
    let data = fs::read_to_string("assets/day22.txt").expect("Could not load file");
    data
}

#[derive(Debug)]
pub enum Direction {
    Down,
    Left,
    Right,
    Up,
}

impl Direction {
    pub fn reverse(&self) -> Self {
        match self {
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
        }
    }

    pub fn turn(&self, turn: Turn) -> Self {
        match turn {
            Turn::Clockwise => match self {
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
            },
            Turn::Counterclockwise => match self {
                Direction::Down => Direction::Right,
                Direction::Right => Direction::Up,
                Direction::Up => Direction::Left,
                Direction::Left => Direction::Down,
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Turn {
    Clockwise,
    Counterclockwise,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Corridor,
}

impl Tile {
    pub fn parse(i: &str) -> IResult<&str, Self> {
        alt((value(Tile::Wall, tag("#")), value(Tile::Corridor, tag("."))))(i)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Motion {
    Pace(u32),
    Turn(Turn),
}

impl Motion {
    pub fn parse(i: &str) -> IResult<&str, Self> {
        alt((
            complete::u32.map(|m| Motion::Pace(m)),
            value(Motion::Turn(Turn::Clockwise), tag("R")),
            value(Motion::Turn(Turn::Counterclockwise), tag("L")),
        ))(i)
    }
}

#[derive(Debug)]
pub struct Jungle(Vec<Vec<Option<Tile>>>);

impl Jungle {
    fn parse_cell(i: &str) -> IResult<&str, Option<Tile>> {
        alt((value(None, tag(" ")), Tile::parse.map(|t| Some(t))))(i)
    }

    pub fn parse_row(i: &str) -> IResult<&str, Vec<Option<Tile>>> {
        many1(Jungle::parse_cell)(i)
    }

    pub fn parse(i: &str) -> IResult<&str, Self> {
        let (i, jungle) = separated_list1(line_ending, Jungle::parse_row)(i)?;
        Ok((i, Jungle(jungle)))
    }

    pub fn find_start(&self) -> Position {
        for (y, row) in self.0.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if let Some(t) = cell {
                    if *t == Tile::Corridor {
                        return Position { x, y };
                    }
                }
            }
        }
        panic!("We messed up, there is no starting point!");
    }

    pub fn height(&self) -> usize {
        self.0.len()
    }

    pub fn width(&self) -> usize {
        self.0[0].len()
    }

    pub fn tile(&self, position: Position) -> Option<Tile> {
        if let Some(row) = self.0.get(position.y) {
            if let Some(tile) = row.get(position.x) {
                return *tile;
                // return self.0[position.y][position.x]
            }
        }
        None
    }
}

impl Display for Jungle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for row in &self.0 {
            for cell in row {
                match cell {
                    Some(tile) => match tile {
                        Tile::Wall => s.push('#'),
                        Tile::Corridor => s.push('.'),
                    },
                    None => s.push('_'),
                }
            }
            s.push('\n');
        }

        write!(f, "{}", s)
    }
}

pub fn parse_motions(i: &str) -> IResult<&str, Vec<Motion>> {
    many1(Motion::parse)(i)
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
pub struct Traveler {
    jungle: Jungle,
    facing: Direction,
    position: Position,
}

impl Traveler {
    pub fn wrap_around(&self) -> Position {
        match self.facing {
            Direction::Down => {
                for i in 0..=self.jungle.height() {
                    if let Some(cell) = self.jungle.0[i].get(self.position.x) {
                        if let Some(_tile) = cell {
                            return Position {
                                x: self.position.x,
                                y: i,
                            };
                        }
                    }
                }
                panic!("There is no valid tiles on column {}!", self.position.x);
            }

            Direction::Left => {
                for i in (0..self.jungle.width()).rev() {
                    if let Some(cell) = self.jungle.0[self.position.y].get(i) {
                        if let Some(_tile) = cell {
                            return Position {
                                x: i,
                                y: self.position.y,
                            };
                        }
                    }
                }
                panic!("There is no valid tiles on row {}!", self.position.y);
            }
            Direction::Right => {
                for i in 0..self.jungle.width() {
                    if let Some(cell) = self.jungle.0[self.position.y].get(i) {
                        if let Some(_tile) = cell {
                            return Position {
                                x: i,
                                y: self.position.y,
                            };
                        }
                    }
                }
                panic!("There is no valid tiles on row {}!", self.position.y);
            }
            Direction::Up => {
                for i in (0..self.jungle.height()).rev() {
                    if let Some(cell) = self.jungle.0[i].get(self.position.x) {
                        if let Some(_tile) = cell {
                            return Position {
                                x: self.position.x,
                                y: i,
                            };
                        }
                    }
                }
                panic!("There is no valid tiles on column {}!", self.position.x);
            }
        }
    }

    pub fn next(&mut self) {
        let next_position = match self.facing {
            Direction::Down => {
                let y = self.position.y + 1;
                let new_position = Position {
                    x: self.position.x,
                    y,
                };

                // Are we going off the map or to a void?
                if y >= self.jungle.height() || self.jungle.tile(new_position).is_none() {
                    self.wrap_around()
                } else {
                    new_position
                }
            }
            Direction::Left => {
                let x = self.position.x.checked_sub(1);

                // Are we going off the map?
                if x.is_none() {
                    self.wrap_around()
                } else {
                    let new_position = Position {
                        x: x.unwrap(),
                        y: self.position.y,
                    };
                    if self.jungle.tile(new_position).is_none() {
                        self.wrap_around()
                    } else {
                        new_position
                    }
                }
            }
            Direction::Right => {
                let x = self.position.x + 1;
                let new_position = Position {
                    x,
                    y: self.position.y,
                };

                // Are we going off the map?
                if x >= self.jungle.width() || self.jungle.tile(new_position).is_none() {
                    self.wrap_around()
                } else {
                    new_position
                }
            }
            Direction::Up => {
                let y = self.position.y.checked_sub(1);

                // Are we going off the map?
                if y.is_none() {
                    self.wrap_around()
                } else {
                    let new_position = Position {
                        x: self.position.x,
                        y: y.unwrap(),
                    };

                    if self.jungle.tile(new_position).is_none() {
                        self.wrap_around()
                    } else {
                        new_position
                    }
                }
            }
        };

        // Is it a corridor?
        if let Some(tile) = self.jungle.tile(next_position) {
            if tile == Tile::Corridor {
                self.position = next_position;
            }
        }
    }
}

pub fn process_input_a(jungle: Jungle, motions: &[Motion]) -> usize {
    let position = jungle.find_start();
    let mut traveler = Traveler {
        jungle,
        position,
        facing: Direction::Right,
    };
    dbg!(&traveler.position);
    for motion in motions {
        match motion {
            Motion::Pace(steps) => {
                for _step in 0..*steps {
                    traveler.next();
                }
                // println!("[{},{}]", &traveler.position.x, &traveler.position.y);
            }
            Motion::Turn(turn) => {
                // println!("Turn: {:?}", turn);
                // print!("Turning from {:?} to ", traveler.facing);
                traveler.facing = traveler.facing.turn(*turn);
                // println!("{:?}", traveler.facing);
            }
        };
    }
    let facing = match traveler.facing {
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Right => 0,
        Direction::Up => 3,
    };
    // dbg!(&traveler.position);
    (1000 * (traveler.position.y + 1)) + (4 * (traveler.position.x + 1)) + facing
}

#[cfg(test)]
mod test {
    use super::*;

    const RAW_INPUT: &'static str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
";

    #[test]
    fn test_day22a_sample() {
        let (i, jungle) = Jungle::parse(RAW_INPUT).unwrap();
        // let (i, _) = alt((line_ending, space1))(i).unwrap();
        let (_, motions) = parse_motions(i.trim()).unwrap();
        let password = process_input_a(jungle, &motions);
        assert_eq!(password, 6032);
    }

    #[test]
    fn test_day22b_sample() {}
}
