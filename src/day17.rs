use itertools::Itertools;
use nom::{branch::alt, bytes::complete::tag, combinator::value, multi::many1, IResult};
use std::{
    cmp::max,
    collections::{HashMap, HashSet},
    fmt::Display,
    fs,
    ops::Add,
};

pub fn day17a() -> String {
    let data = fs::read_to_string("assets/day17.txt").expect("Could not load file");
    let rock_count = process_input_a(&data, 2022);
    rock_count.to_string()
    // 3266 is too high
}

pub fn day17b() -> String {
    let data = fs::read_to_string("assets/day17.txt").expect("Could not load file");
    data
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct Coordinate {
    x: i32,
    y: i32,
}

impl Add for Coordinate {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.x, self.y)
    }
}

impl Coordinate {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Down,
    Left,
    Right,
    Up,
}

impl Direction {
    pub fn parse(i: &str) -> IResult<&str, Self> {
        alt((
            value(Direction::Left, tag("<")),
            value(Direction::Right, tag(">")),
        ))(i)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    I,
    L,
    Minus,
    Plus,
    Square,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rock {
    shape: Shape,
    height: i32,
    formation: HashSet<Coordinate>,
}

impl Display for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formation = self
            .formation
            .iter()
            .map(|&c| c)
            .collect::<Vec<Coordinate>>();
        formation.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
        let results = formation
            .iter()
            .map(|c| format!("{c}"))
            .collect::<Vec<String>>();
        let s = results.join("");
        write!(f, "{}", s)
    }
}

impl Rock {
    fn new_minus() -> Self {
        let mut formation = HashSet::new();
        formation.insert(Coordinate::new(0, 0));
        formation.insert(Coordinate::new(1, 0));
        formation.insert(Coordinate::new(2, 0));
        formation.insert(Coordinate::new(3, 0));
        Rock {
            shape: Shape::Minus,
            height: 1,
            formation,
        }
    }

    fn new_plus() -> Self {
        let mut formation = HashSet::new();
        formation.insert(Coordinate::new(0, 1));
        formation.insert(Coordinate::new(1, 0));
        formation.insert(Coordinate::new(1, 1));
        formation.insert(Coordinate::new(1, 2));
        formation.insert(Coordinate::new(2, 1));
        Rock {
            shape: Shape::Plus,
            height: 3,
            formation,
        }
    }

    fn new_l() -> Self {
        let mut formation = HashSet::new();
        formation.insert(Coordinate::new(0, 0));
        formation.insert(Coordinate::new(1, 0));
        formation.insert(Coordinate::new(2, 0));
        formation.insert(Coordinate::new(2, 1));
        formation.insert(Coordinate::new(2, 2));
        Rock {
            shape: Shape::L,
            height: 3,
            formation,
        }
    }

    fn new_i() -> Self {
        let mut formation = HashSet::new();
        formation.insert(Coordinate::new(0, 0));
        formation.insert(Coordinate::new(0, 1));
        formation.insert(Coordinate::new(0, 2));
        formation.insert(Coordinate::new(0, 3));
        Rock {
            shape: Shape::I,
            height: 4,
            formation,
        }
    }

    fn new_square() -> Self {
        let mut formation = HashSet::new();
        formation.insert(Coordinate::new(0, 0));
        formation.insert(Coordinate::new(1, 0));
        formation.insert(Coordinate::new(0, 1));
        formation.insert(Coordinate::new(1, 1));
        Rock {
            shape: Shape::Square,
            height: 2,
            formation,
        }
    }

    fn offset(&mut self, offset: &Coordinate) {
        let offset_formation = self
            .formation
            .iter()
            .map(|&c| c + *offset)
            .collect::<HashSet<Coordinate>>();
        self.formation = offset_formation;
    }

    pub fn new(shape: Shape, offset: Option<Coordinate>) -> Self {
        let mut rock = match shape {
            Shape::I => Rock::new_i(),
            Shape::L => Rock::new_l(),
            Shape::Minus => Rock::new_minus(),
            Shape::Plus => Rock::new_plus(),
            Shape::Square => Rock::new_square(),
        };
        if let Some(offset) = offset {
            rock.offset(&offset);
        }
        rock
    }

    pub fn shift(&mut self, direction: &Direction) {
        let offset = match direction {
            Direction::Left => Coordinate::new(-1, 0),
            Direction::Right => Coordinate::new(1, 0),
            Direction::Down => Coordinate::new(0, -1),
            Direction::Up => Coordinate::new(0, 1),
        };
        self.offset(&offset);
    }

    pub fn can_shift(&self, direction: &Direction) -> bool {
        match direction {
            Direction::Left => self.formation.iter().all(|c| c.x != 0),
            Direction::Right => self.formation.iter().all(|c| c.x != 6),
            Direction::Down => panic!("Must not shift down."),
            Direction::Up => panic!("Must not shift up."),
        }
    }
}

#[derive(Debug)]
pub struct Chamber {
    bottom_line: HashSet<Coordinate>,
}

impl Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut bottom_line = self
            .bottom_line
            .iter()
            .map(|&c| c)
            .collect::<Vec<Coordinate>>();
        bottom_line.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
        let results = bottom_line
            .iter()
            .map(|c| format!("{c}"))
            .collect::<Vec<String>>();
        let s = results.join("");
        write!(f, "{}", s)
    }
}

impl Chamber {
    pub fn new() -> Self {
        let mut bottom_line = HashSet::new();
        for i in 0..7 {
            bottom_line.insert(Coordinate::new(i, 0));
        }
        Chamber { bottom_line }
    }

    pub fn highest_rock(&self) -> i32 {
        self.bottom_line.iter().map(|c| c.y).max().unwrap()
    }

    pub fn lowest_rock(&self) -> i32 {
        self.bottom_line.iter().map(|c| c.y).min().unwrap()
    }

    pub fn update(&mut self, rock: &Rock) {
        // // Index the rock parts.
        // let mut rock_parts: HashMap<i32, Coordinate> = HashMap::new();
        // for i in 0..7 {
        //     let mut items_in_x = rock
        //         .formation
        //         .iter()
        //         .filter(|&&c| c.x == i)
        //         .map(|&c| c)
        //         .collect::<Vec<Coordinate>>();
        //     items_in_x.sort();
        //     if let Some(c) = items_in_x.last() {
        //         rock_parts.insert(i, *c);
        //     }
        // }

        // // Find the highest between the rock and the bottom line.
        // let mut bottom_line = HashSet::new();
        // for bottom_line_part in self.bottom_line.drain() {
        //     let y: i32;
        //     if let Some(rock_part) = rock_parts.get(&bottom_line_part.x) {
        //         y = max(rock_part.y, bottom_line_part.y);
        //     } else {
        //         y = bottom_line_part.y;
        //     }
        //     bottom_line.insert(Coordinate::new(bottom_line_part.x, y));
        // }
        // self.bottom_line = bottom_line;
        // assert_eq!(self.bottom_line.len(), 7);
        for rock_part in &rock.formation {
            self.bottom_line.insert(*rock_part);
        }
    }
}

pub fn parse_motions(i: &str) -> IResult<&str, Vec<Direction>> {
    many1(Direction::parse)(i)
}

pub fn process_input_a(i: &str, limit: usize) -> i32 {
    let mut chamber = Chamber::new();
    let (_, motions) = parse_motions(i).unwrap();
    let rocks: Vec<Rock> = vec![
        Rock::new(Shape::Minus, None),
        Rock::new(Shape::Plus, None),
        Rock::new(Shape::L, None),
        Rock::new(Shape::I, None),
        Rock::new(Shape::Square, None),
    ];

    // Pick a move.
    let mut motion_iter = motions.iter().cycle();

    // Let the rock fall.
    let mut rock_count = 0;
    for rock in rocks.iter().cycle() {
        // Stop when hitting the limit.
        if rock_count >= limit {
            return chamber.highest_rock();
        }

        // Bring the rock in.
        rock_count += 1;
        let mut r = rock.clone();
        r.offset(&Coordinate::new(2, chamber.highest_rock() + 1 + 3));

        while let Some(motion) = motion_iter.next() {
            // Try to shift the rock laterally.
            if r.can_shift(&motion) {
                r.shift(motion);
            }

            // Move it down.
            r.shift(&Direction::Down);

            // Sanity checks for debugging.
            if r.formation.iter().any(|c| c.y < 0) {
                dbg!(&rock_count);
                dbg!(&r);
                dbg!(&chamber);
                panic!("Cannot go that low!");
            }
            if r.formation.iter().any(|c| c.x < 0 || c.x > 6) {
                dbg!(&r);
                dbg!(&chamber);
                panic!("Cannot shift that far!");
            }

            // println!("{r}");

            // Does it intersect with the bottom?
            if r.formation.intersection(&chamber.bottom_line).count() > 0 {
                // println!("Intersection!");
                // Then move it back up.
                r.shift(&Direction::Up);
                // println!("{r}");

                // Update chamber's bottom line
                chamber.update(&r);
                // println!("Bottom line: {chamber}");

                // Move to the next rock.
                break;
            }

            if r.formation.iter().any(|c| c.y <= chamber.lowest_rock()) {
                dbg!(&chamber.highest_rock());
                dbg!(&rock_count);
                dbg!(&r);
                dbg!(&chamber);
                panic!("We missed an intersection!");
            }
        }
    }

    chamber.highest_rock()
}

#[cfg(test)]
mod test {
    use super::*;

    const RAW_INPUT: &'static str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_day17a_sample() {
        let rock_count = process_input_a(RAW_INPUT, 2022);
        assert_eq!(rock_count, 3068);
    }

    #[test]
    fn test_day17b_sample() {}

    #[test]
    fn test_rock_offset() {
        let mut rock = Rock::new(Shape::Minus, None);
        rock.offset(&Coordinate::new(2, 2));
        let mut formation = HashSet::new();
        formation.insert(Coordinate::new(2, 2));
        formation.insert(Coordinate::new(3, 2));
        formation.insert(Coordinate::new(4, 2));
        formation.insert(Coordinate::new(5, 2));
        assert_eq!(
            rock,
            Rock {
                shape: Shape::Minus,
                height: 1,
                formation
            }
        );
    }

    #[test]
    fn test_rock_can_shift() {
        let mut rock = Rock::new(Shape::Minus, Some(Coordinate::new(2, 2)));
        let can_shift = rock.can_shift(&Direction::Left);
        assert_eq!(can_shift, true);
        rock.shift(&Direction::Left);
        let mut formation = HashSet::new();
        formation.insert(Coordinate::new(1, 2));
        formation.insert(Coordinate::new(2, 2));
        formation.insert(Coordinate::new(3, 2));
        formation.insert(Coordinate::new(4, 2));
        assert_eq!(
            rock,
            Rock {
                shape: Shape::Minus,
                height: 1,
                formation
            }
        );
        rock.shift(&Direction::Down);
        let mut formation = HashSet::new();
        formation.insert(Coordinate::new(1, 1));
        formation.insert(Coordinate::new(2, 1));
        formation.insert(Coordinate::new(3, 1));
        formation.insert(Coordinate::new(4, 1));
        assert_eq!(
            rock,
            Rock {
                shape: Shape::Minus,
                height: 1,
                formation
            }
        );
    }

    #[test]
    fn test_rock_shift_down() {
        let mut rock = Rock::new(Shape::Minus, Some(Coordinate::new(2, 2)));
        rock.shift(&Direction::Down);
        let mut formation = HashSet::new();
        formation.insert(Coordinate::new(2, 1));
        formation.insert(Coordinate::new(3, 1));
        formation.insert(Coordinate::new(4, 1));
        formation.insert(Coordinate::new(5, 1));
        assert_eq!(
            rock,
            Rock {
                shape: Shape::Minus,
                height: 1,
                formation
            }
        );
    }

    #[test]
    fn test_chamber_update() {
        let mut chamber = Chamber::new();
        let rock = Rock::new(Shape::Minus, Some(Coordinate::new(2, 1)));
        chamber.update(&rock);
        assert_eq!(chamber.highest_rock(), 1);
        let rock = Rock::new(
            Shape::Plus,
            Some(Coordinate::new(2, chamber.highest_rock() + 1)),
        );
        // dbg!(&rock);
        chamber.update(&rock);
        // dbg!(&chamber.top_line);
        assert_eq!(chamber.highest_rock(), 4);
    }
}
