use std::{collections::HashSet, fs};

pub const PACKET_MARKER_SIZE: usize = 4;
pub const PACKET_MESSAGE_SIZE: usize = 14;

pub fn day06a() -> String {
    let data = fs::read_to_string("assets/day06.txt").expect("Could not load file");
    let marker_pos = process_input_a(&data, PACKET_MARKER_SIZE);
    marker_pos.to_string()
}

pub fn day06b() -> String {
    let data = fs::read_to_string("assets/day06.txt").expect("Could not load file");
    let marker_pos = process_input_a(&data, PACKET_MESSAGE_SIZE);
    marker_pos.to_string()
}

pub fn process_input_a(input: &str, size: usize) -> usize {
    let sequence = input.chars().collect::<Vec<char>>();
    for (i, subsequence) in sequence.windows(size).enumerate() {
        let hash = subsequence.iter().map(|&c| c).collect::<HashSet<char>>();
        if hash.len() == size {
            return i + size;
        }
    }
    panic!("No valid marker found");
}

#[cfg(test)]
mod test {
    use super::*;

    const RAW_INPUT: &'static str = "bvwbjplbgvbhsrlpgdmjqwftvncz";

    #[test]
    fn test_day06a_sample() {
        let marker_pos = process_input_a(RAW_INPUT, PACKET_MARKER_SIZE);
        assert_eq!(marker_pos, 5)
    }

    #[test]
    fn test_day06b_sample() {
        let marker_pos = process_input_a(RAW_INPUT, PACKET_MESSAGE_SIZE);
        assert_eq!(marker_pos, 23)
    }
}
