// Day 14: Docking Data

use std::{collections::HashMap, str::FromStr};

use shared::prelude::*;

const BITS: usize = 36;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Bitmask(Vec<Option<bool>>);

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct MemInstruction {
    address: u64,
    value: u64,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Instruction {
    SetBitmask(Bitmask),
    SetValue(MemInstruction),
}

pub struct Memory(HashMap<u64, u64>);

lazy_static! {
    static ref MEM_INDEX_REGEX: Regex = Regex::new(r"^mem\[([0-9]+)\]$").unwrap();
    static ref TEST_INPUT: Vec<Instruction> = vec![
        "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
        "mem[8] = 11",
        "mem[7] = 101",
        "mem[8] = 0",
    ]
    .into_iter()
    .map(Instruction::from_str)
    .collect::<Result<_, _>>()
    .unwrap();
    static ref PUZZLE_INPUT: Vec<Instruction> =
        puzzle_input::lines(include_str!("puzzle_input.txt"))
            .into_iter()
            .map(Instruction::from_str)
            .collect::<Result<_, _>>()
            .unwrap();
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(" = ").collect();
        match split[0] {
            "mask" => Ok(Instruction::SetBitmask(split[1].parse()?)),
            memstr if memstr.starts_with("mem[") => {
                let captures = MEM_INDEX_REGEX
                    .captures(memstr)
                    .ok_or(anyhow!("bad index"))?;
                Ok(Instruction::SetValue(MemInstruction {
                    address: captures[1].parse()?,
                    value: split[1].parse()?,
                }))
            }
            _ => Err(anyhow!("bad instruction")),
        }
    }
}

impl FromStr for Bitmask {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != BITS {
            return Err(anyhow!("Wrong size"));
        }

        Ok(Bitmask(
            s.chars()
                .map(|x| match x {
                    'X' => Ok(None),
                    '1' => Ok(Some(true)),
                    '0' => Ok(Some(false)),
                    _ => Err(anyhow!("bad bitmask")),
                })
                .collect::<Result<_, _>>()?,
        ))
    }
}

impl Bitmask {
    pub fn apply(&self, original: u64) -> u64 {
        (0..BITS).fold(original, |num, bit| {
            let this_bit = 1_u64 << bit;
            let mask = self.0[self.0.len() - 1 - bit];
            match mask {
                Some(true) => num | this_bit,
                Some(false) => num & !this_bit,
                None => num,
            }
        })
    }
}

impl Memory {
    pub fn new() -> Memory {
        Memory(HashMap::new())
    }

    pub fn sum_values(&self) -> u64 {
        self.0.values().sum()
    }
}

fn flatten_instructions(
    instructions: &[Instruction],
) -> anyhow::Result<Vec<(Bitmask, MemInstruction)>> {
    instructions
        .iter()
        .scan(None, |last_bitmask, instruction| {
            match instruction {
                Instruction::SetBitmask(bitmask) => {
                    *last_bitmask = Some(bitmask);
                }
                Instruction::SetValue(_) => {}
            };

            match last_bitmask {
                Some(last_bitmask) => Some(Ok((last_bitmask.clone(), instruction))),
                None => Some(Err(anyhow!("memset without bitmask"))),
            }
        })
        .filter_map(
            |instruction_pair| -> Option<anyhow::Result<(Bitmask, MemInstruction)>> {
                instruction_pair.map_or_else(
                    |err| Some(Err(err)),
                    |(bitmask, instruction)| match instruction {
                        Instruction::SetBitmask(_) => None,
                        Instruction::SetValue(value) => Some(Ok((bitmask, *value))),
                    },
                )
            },
        )
        .collect()
}

pub fn run_instructions(instructions: &[Instruction]) -> anyhow::Result<Memory> {
    Ok(flatten_instructions(instructions)?.iter().fold(
        Memory::new(),
        |Memory(mut memory_map), (bitmask, mem_instruction)| {
            memory_map.insert(
                mem_instruction.address,
                bitmask.apply(mem_instruction.value),
            );
            Memory(memory_map)
        },
    ))
}

#[cfg(test)]
mod part_one {
    use super::*;

    #[test]
    fn test_basic() {
        let memory =
            run_instructions(&TEST_INPUT.iter().take(2).cloned().collect::<Vec<_>>()).unwrap();

        assert_eq!(memory.0.get(&8).unwrap(), &73);
    }

    #[test]
    fn test_case() {
        assert_eq!(
            run_instructions(TEST_INPUT.as_slice())
                .unwrap()
                .sum_values(),
            165
        );
    }

    #[test]
    fn answer() {
        assert_eq!(
            run_instructions(PUZZLE_INPUT.as_slice())
                .unwrap()
                .sum_values(),
            13556564111697
        );
    }
}

// #[cfg(test)]
// mod part_two {
//     use super::*;
//     #[test]
//     fn test_cases() {}
//     #[test]
//     fn answer() {}
// }
