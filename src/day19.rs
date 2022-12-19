use std::fs;

pub fn day19a() -> String {
    let data = fs::read_to_string("assets/day19.txt").expect("Could not load file");
    data
}

pub fn day19b() -> String {
    let data = fs::read_to_string("assets/day19.txt").expect("Could not load file");
    data
}

#[cfg(test)]
mod test {
    use super::*;

    const RAW_INPUT: &'static str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
";

    #[test]
    fn test_day19a_sample() {}

    #[test]
    fn test_day19b_sample() {}
}
