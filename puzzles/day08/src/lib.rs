// Day 8: Handheld Halting

use std::str::FromStr;

use shared::prelude::*;

#[derive(Debug, Eq, PartialEq)]
pub enum OperationCode {
    Nop,
    Acc,
    Jmp,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Instruction {
    operation: OperationCode,
    argument: i32,
}

lazy_static! {
    static ref PUZZLE_INPUT: Vec<&'static str> =
        puzzle_input::lines(include_str!("puzzle_input.txt"));
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<_> = s.split(" ").collect();
        let operation = match split.get(0) {
            Some(&"nop") => Ok(OperationCode::Nop),
            Some(&"acc") => Ok(OperationCode::Acc),
            Some(&"jmp") => Ok(OperationCode::Jmp),
            Some(x) => Err(anyhow!("Unrecognized operation code: {}", x)),
            None => Err(anyhow!("Missing operation code")),
        }?;
        let argument = match split.get(1) {
            Some(num) => Ok(num.parse()?),
            None => Err(anyhow!("Missing argument")),
        }?;

        Ok(Instruction {
            operation,
            argument,
        })
    }
}

#[cfg(test)]
mod part_one {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            Instruction::from_str("acc -99").unwrap(),
            Instruction {
                operation: OperationCode::Acc,
                argument: -99
            }
        );
        assert_eq!(
            Instruction::from_str("nop +0").unwrap(),
            Instruction {
                operation: OperationCode::Nop,
                argument: 0
            }
        );
        assert_eq!(
            Instruction::from_str("jmp +4").unwrap(),
            Instruction {
                operation: OperationCode::Jmp,
                argument: 4
            }
        );
    }

    // #[test]
    // fn answer() {
    //     assert_eq!(*PUZZLE_INPUT, Vec::<String>::new());
    // }
}

// #[cfg(test)]
// mod part_two {
//     use super::*;
//     #[test]
//     fn test_cases() {}
//     #[test]
//     fn answer() {}
// }
