use std::collections::HashSet;
use std::fs;

pub fn day09a() -> String {
    let data = fs::read_to_string("assets/day09.txt").expect("Could not load file");
    let motions = parse_input_a(&data);
    let visited = process_input(&motions);
    visited.to_string()
}

pub fn day09b() -> String {
    let data = fs::read_to_string("assets/day09.txt").expect("Could not load file");
    data
}

#[derive(Debug)]
pub enum Direction {
    U,
    D,
    L,
    R,
}

impl From<&str> for Direction {
    fn from(item: &str) -> Self {
        match item {
            "U" => Direction::U,
            "D" => Direction::D,
            "L" => Direction::L,
            "R" => Direction::R,
            _ => panic!("Invalid direction: {item}"),
        }
    }
}

#[derive(Debug)]
pub struct Motion {
    direction: Direction,
    steps: i32,
}

impl From<&str> for Motion {
    fn from(item: &str) -> Self {
        let split = item.split_whitespace().collect::<Vec<&str>>();
        let direction = split.first().map(|&d| Direction::from(d)).unwrap();
        let steps = split.last().unwrap().parse::<i32>().unwrap();
        Motion { direction, steps }
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn up(&mut self) {
        self.y += 1;
    }

    pub fn down(&mut self) {
        self.y -= 1;
    }

    pub fn left(&mut self) {
        self.x -= 1;
    }

    pub fn right(&mut self) {
        self.x += 1;
    }

    pub fn adjacent(&self, other: &Position) -> bool {
        ((self.x - other.x).abs() <= 1) && ((self.y - other.y).abs() <= 1)
    }

    pub fn same_row(&self, other: &Position) -> bool {
        self.x == other.x
    }

    pub fn same_column(&self, other: &Position) -> bool {
        self.y == other.y
    }

    pub fn step(&mut self, direction: &Direction) {
        match direction {
            Direction::U => self.up(),
            Direction::D => self.down(),
            Direction::L => self.left(),
            Direction::R => self.right(),
        }
    }
}

// pub struct Rope {
//     tail: Position,
//     head: Position,
//     visited: HashSet<Position>,
// }

pub fn parse_input_a(input: &str) -> Vec<Motion> {
    input.lines().map(Motion::from).collect::<Vec<Motion>>()
}

pub fn process_input(motions: &[Motion]) -> usize {
    // Initialize the motion.
    let mut tail = Position { x: 0, y: 0 };
    let mut head = Position { x: 0, y: 0 };
    let mut visited: HashSet<Position> = HashSet::new();
    visited.insert(tail.clone());

    for motion in motions {
        for _ in 0..motion.steps {
            // dbg!(&motion);
            head.step(&motion.direction);
            // dbg!(&head, &tail, &head.adjacent(&tail));
            if head.adjacent(&tail) {
                continue;
            }
            if !head.same_row(&tail) || !head.same_column(&tail) {
                let horizontal_diff = (head.x - tail.x).signum();
                let vertical_diff = (head.y - tail.y).signum();
                tail.x += horizontal_diff;
                tail.y += vertical_diff;
            } else {
                tail.step(&motion.direction);
            }

            visited.insert(tail.clone());
            // dbg!(&tail);
        }
    }

    visited.len()
}

#[cfg(test)]
mod test {
    use super::*;

    const RAW_INPUT: &'static str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

    #[test]
    fn test_day09a_sample() {
        let motions = parse_input_a(RAW_INPUT);
        let visited = process_input(&motions);
        assert_eq!(visited, 13);
    }

    #[test]
    fn test_day09b_sample() {}
}
