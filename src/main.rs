// use aoc2021::day01::{day01a, day01b};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let problem = args.get(1).map(|s| s.as_str()).unwrap_or("None");
    let result = match problem {
        // "day01a" => day01a(),
        _ => "We haven't solved that yet".to_string(),
    };
    println!("{}", result);
}
