use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while},
    character::{
        complete::{alphanumeric1, char, space0, u32, u8},
        is_digit,
    },
    error::{Error, ErrorKind},
    multi::separated_list1,
    sequence::preceded,
    Finish, IResult,
};
use std::{fs, str::FromStr};

pub fn day11a() -> String {
    let data = fs::read_to_string("assets/day11.txt").expect("Could not load file");
    data
}

pub fn day11b() -> String {
    let data = fs::read_to_string("assets/day11.txt").expect("Could not load file");
    data
}

pub fn parse_input_a(input: &str) -> Vec<Monkey> {
    let raw_monkeys = input
        .split("\n\n")
        .map(String::from)
        .collect::<Vec<String>>();
    raw_monkeys
        .iter()
        .map(|l| l.trim())
        .map(|l| l.parse::<Monkey>().unwrap())
        .collect::<Vec<Monkey>>()
}

#[derive(Debug, PartialEq, Eq)]
pub struct Items(Vec<u32>);

impl Items {
    pub fn parse(i: &str) -> IResult<&str, Items> {
        //  Starting items: 84, 66, 62, 69, 88, 91, 91
        let (i, _) = take_while(|c| !is_digit(c as u8))(i)?;
        let (i, items) = separated_list1(tag(", "), u32)(i)?;
        Ok((i, Items(items)))
    }
}

impl FromStr for Items {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Items::parse(s).finish() {
            Ok((_remaining, items)) => Ok(items),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operation {
    Multiply(u32),
    Add(u32),
    Square,
}

impl Operation {
    pub fn parse(i: &str) -> IResult<&str, Operation> {
        // Operation: new = old * 11
        let (i, _) = alt((take_until("*"), take_until("+")))(i)?;
        let (i, symbol) = alt((char('+'), char('*')))(i)?;
        let (i, amount) = preceded(tag(" "), alphanumeric1)(i)?;
        let op = match amount {
            "old" => Operation::Square,
            _ => match symbol {
                '+' => Operation::Add(amount.parse::<u32>().unwrap()),
                '*' => Operation::Multiply(amount.parse::<u32>().unwrap()),
                _ => {
                    return Err(nom::Err::Error(Error {
                        input: "Invalid symbol",
                        code: ErrorKind::Char,
                    }))
                }
            },
        };
        Ok((i, op))
    }
}

impl FromStr for Operation {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Operation::parse(s).finish() {
            Ok((_remaining, operation)) => Ok(operation),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Test {
    amount: u32,
    truthy: u32,
    falsy: u32,
}

impl Test {
    pub fn parse(i: &str) -> IResult<&str, Test> {
        //   Test: divisible by 2
        //     If true: throw to monkey 4
        //     If false: throw to monkey 7
        let (i, _) = take_while(|c| !is_digit(c as u8))(i)?;
        let (i, amount) = u32(i)?;
        let (i, _) = take_while(|c| !is_digit(c as u8))(i)?;
        let (i, truthy) = u32(i)?;
        let (i, _) = take_while(|c| !is_digit(c as u8))(i)?;
        let (i, falsy) = u32(i)?;
        Ok((
            i,
            Test {
                amount,
                truthy,
                falsy,
            },
        ))
    }
}

impl FromStr for Test {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Test::parse(s).finish() {
            Ok((_remaining, test)) => Ok(test),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Monkey {
    id: u8,
    items: Items,
    operation: Operation,
    test: Test,
    inspected_items: u32,
}

impl Monkey {
    pub fn parse(i: &str) -> IResult<&str, Monkey> {
        // Monkey 0:
        let (i, _) = space0(i)?;
        let (i, _) = tag("Monkey ")(i)?;
        let (i, id) = u8(i)?;
        let (i, items) = Items::parse(i)?;
        let (i, operation) = Operation::parse(i)?;
        let (i, test) = Test::parse(i)?;

        Ok((
            i,
            Monkey {
                id,
                items,
                operation,
                test,
                inspected_items: 0,
            },
        ))
    }
}

impl FromStr for Monkey {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Monkey::parse(s).finish() {
            Ok((_remaining, monkey)) => Ok(monkey),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const RAW_INPUT: &'static str = "Monkey 0:
    Starting items: 79, 98
    Operation: new = old * 19
    Test: divisible by 23
      If true: throw to monkey 2
      If false: throw to monkey 3

  Monkey 1:
    Starting items: 54, 65, 75, 74
    Operation: new = old + 6
    Test: divisible by 19
      If true: throw to monkey 2
      If false: throw to monkey 0

  Monkey 2:
    Starting items: 79, 60, 97
    Operation: new = old * old
    Test: divisible by 13
      If true: throw to monkey 1
      If false: throw to monkey 3

  Monkey 3:
    Starting items: 74
    Operation: new = old + 3
    Test: divisible by 17
      If true: throw to monkey 0
      If false: throw to monkey 1
";

    #[test]
    fn test_day11a_sample() {
        let monkeys = parse_input_a(RAW_INPUT);
        dbg!(&monkeys);
    }

    #[test]
    fn test_day11b_sample() {}

    #[test]
    fn test_parse_items() {
        assert_eq!(
            "  Starting items: 84, 66, 62, 69, 88, 91, 91".parse::<Items>(),
            Ok(Items(vec![84, 66, 62, 69, 88, 91, 91]))
        );
    }

    #[test]
    fn test_parse_operation() {
        assert_eq!(
            "  Operation: new = old * 11".parse::<Operation>(),
            Ok(Operation::Multiply(11))
        );
        assert_eq!(
            "  Operation: new = old + 1".parse::<Operation>(),
            Ok(Operation::Add(1))
        );
    }

    #[test]
    fn test_parse_test() {
        assert_eq!(
            "   Test: divisible by 2\n     If true: throw to monkey 4\n     If false: throw to monkey 7".parse::<Test>(),
            Ok(Test { amount:2, truthy:4, falsy:7})
        );
    }

    #[test]
    fn test_parse_monkey() {
        let monkey_str = "  Monkey 0:
        Starting items: 84, 66, 62, 69, 88, 91, 91
        Operation: new = old * 11
        Test: divisible by 2
          If true: throw to monkey 4
          If false: throw to monkey 7
        ";
        assert_eq!(
            monkey_str.parse::<Monkey>(),
            Ok(Monkey {
                id: 0,
                items: Items(vec![84, 66, 62, 69, 88, 91, 91]),
                operation: Operation::Multiply(11),
                test: Test {
                    amount: 2,
                    truthy: 4,
                    falsy: 7
                },
                inspected_items: 0
            })
        );
    }
}
