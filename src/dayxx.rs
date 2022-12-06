use std::fs;

pub fn dayxxa() -> String {
    let data = fs::read_to_string("assets/dayxx.txt").expect("Could not load file");
    data
}

pub fn dayxxb() -> String {
    let data = fs::read_to_string("assets/dayxx.txt").expect("Could not load file");
    data
}

#[cfg(test)]
mod test {
    use super::*;

    const RAW_INPUT: &'static str = "
";

    #[test]
    fn test_dayxxa_sample() {}

    #[test]
    fn test_dayxxb_sample() {}
}
