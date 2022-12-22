use std::fs;

pub fn day21a() -> String {
    let data = fs::read_to_string("assets/day21.txt").expect("Could not load file");
    data
}

pub fn day21b() -> String {
    let data = fs::read_to_string("assets/day21.txt").expect("Could not load file");
    data
}

#[cfg(test)]
mod test {
    use super::*;

    const RAW_INPUT: &'static str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";

    #[test]
    fn test_day21a_sample() {}

    #[test]
    fn test_day21b_sample() {}
}
