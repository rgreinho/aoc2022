use std::fs;

pub fn day10a() -> String {
    let data = fs::read_to_string("assets/day10.txt").expect("Could not load file");
    let instructions = parse_input_a(&data);
    let combined_signal_strenghs = process_input_a(&instructions);
    combined_signal_strenghs.to_string()
}

pub fn day10b() -> String {
    let data = fs::read_to_string("assets/day10.txt").expect("Could not load file");
    let instructions = parse_input_a(&data);
    process_input_b(&instructions);
    0.to_string()
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Noop,
    Addx(i32),
}

impl From<&str> for Instruction {
    fn from(item: &str) -> Self {
        if item == "noop" {
            return Instruction::Noop;
        }
        let split = item
            .split_whitespace()
            .map(String::from)
            .collect::<Vec<String>>();
        let value = split.last().unwrap().parse::<i32>().unwrap();
        Instruction::Addx(value)
    }
}

pub fn parse_input_a(input: &str) -> Vec<Instruction> {
    // input.lines().map(Instruction::from).collect()
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in input.lines() {
        let instruction = Instruction::from(line);
        match instruction {
            Instruction::Noop => instructions.push(instruction),
            Instruction::Addx(_) => {
                instructions.push(Instruction::Noop);
                instructions.push(instruction);
            }
        }
    }
    instructions
}

pub fn process_input_a(instructions: &[Instruction]) -> i32 {
    let mut registry = 1;
    let mut special_cycle = 20;
    let mut signal_strength = 0;

    for (cycle, instruction) in instructions.iter().enumerate() {
        let cycle1 = cycle + 1;
        if cycle1 == special_cycle {
            let current_signal_strength = cycle1 as i32 * registry;
            signal_strength += current_signal_strength;
            special_cycle += 40;
            println!("[{cycle1}], {registry}, {current_signal_strength}, {signal_strength}");
        }
        if let Instruction::Addx(value) = instruction {
            registry += value;
        }
    }
    signal_strength
}

pub fn process_input_b(instructions: &[Instruction]) {
    let mut registry: i32 = 1;

    for (cycle, instruction) in instructions.iter().enumerate() {
        let cyclemod40 = cycle % 40;
        if cyclemod40 == 0 {
            println!("");
        }
        if (cyclemod40 as i32) < (registry - 1) || (cyclemod40 as i32) > (registry + 1) {
            print!(".");
        } else {
            print!("#")
        }

        if let Instruction::Addx(value) = instruction {
            registry += value;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const RAW_INPUT: &'static str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";

    #[test]
    fn test_day10a_sample() {
        let instructions = parse_input_a(RAW_INPUT);
        let combined_signal_strenghs = process_input_a(&instructions);
        assert_eq!(combined_signal_strenghs, 13140);
    }

    #[test]
    fn test_day10b_sample() {
        let instructions = parse_input_a(RAW_INPUT);
        process_input_b(&instructions);
    }
}
