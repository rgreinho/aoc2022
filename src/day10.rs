use std::fs;

pub fn day10a() -> String {
    let data = fs::read_to_string("assets/day10.txt").expect("Could not load file");
    data
}

pub fn day10b() -> String {
    let data = fs::read_to_string("assets/day10.txt").expect("Could not load file");
    data
}

#[cfg(test)]
mod test {
    use super::*;

    const RAW_INPUT: &'static str = "
";

    #[test]
    fn test_day10a_sample() {}

    #[test]
    fn test_day10b_sample() {}
}
