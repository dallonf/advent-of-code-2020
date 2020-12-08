// Day 8: Handheld Halting

use std::{collections::HashSet, str::FromStr};

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
    static ref TEST_INPUT: Vec<Instruction> = puzzle_input::lines(include_str!("test_input.txt"))
        .into_iter()
        .map(Instruction::from_str)
        .collect::<anyhow::Result<Vec<_>>>()
        .unwrap();
    static ref PUZZLE_INPUT: Vec<Instruction> =
        puzzle_input::lines(include_str!("puzzle_input.txt"))
            .into_iter()
            .map(Instruction::from_str)
            .collect::<anyhow::Result<Vec<_>>>()
            .unwrap();
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

#[derive(Debug)]
pub enum ExecutionResult {
    Loop(i32),
    Terminate(i32),
}

pub fn execute(instructions: &[Instruction]) -> anyhow::Result<ExecutionResult> {
    fn step(
        instructions: &[Instruction],
        mut visit_index: usize,
        mut accumulator: i32,
        mut indexes_already_run: HashSet<usize>,
    ) -> anyhow::Result<ExecutionResult> {
        if visit_index == instructions.len() {
            return Ok(ExecutionResult::Terminate(accumulator));
        }

        if indexes_already_run.contains(&visit_index) {
            return Ok(ExecutionResult::Loop(accumulator));
        }
        indexes_already_run.insert(visit_index);

        let instruction = instructions.get(visit_index).map_or_else(
            || Err(anyhow!("No instruction at index: {}", visit_index)),
            |x| Ok(x),
        )?;

        match instruction.operation {
            OperationCode::Nop => {
                visit_index += 1;
            }
            OperationCode::Acc => {
                accumulator += instruction.argument;
                visit_index += 1;
            }
            OperationCode::Jmp => {
                let new_index = visit_index as i32 + instruction.argument;
                if new_index < 0 || new_index > instructions.len() as i32 {
                    return Err(anyhow!("jumped to index out of bounds: {}", new_index));
                }
                visit_index = new_index as usize;
            }
        };

        step(instructions, visit_index, accumulator, indexes_already_run)
    }

    step(instructions, 0, 0, HashSet::new())
}

pub fn get_accumulator_before_loop(instructions: &[Instruction]) -> anyhow::Result<i32> {
    match execute(instructions) {
        Ok(ExecutionResult::Loop(accumulator)) => Ok(accumulator),
        Ok(other) => Err(anyhow!("Unexpected result: {:?}", other)),
        Err(err) => Err(err),
    }
}

// pub fn fix_program(instructions: &[Instruction]) -> anyhow::Result<i32> {
//     Ok(0)
// }

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

    #[test]
    fn test_case() {
        assert_eq!(get_accumulator_before_loop(&TEST_INPUT).unwrap(), 5);
    }

    #[test]
    fn answer() {
        assert_eq!(get_accumulator_before_loop(&PUZZLE_INPUT).unwrap(), 1930);
    }
}

// #[cfg(test)]
// mod part_two {
//     use super::*;
//     #[test]
//     fn test_case() {
//         assert_eq!(fix_program(&TEST_INPUT).unwrap(), 6);
//     }
//     // #[test]
//     // fn answer() {}
// }
