// Day 12: Rain Risk

use std::str::FromStr;

use shared::prelude::*;

pub mod part_one;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Direction(Direction, i32),
    Turn(i32),
    Forward(i32),
}

lazy_static! {
    static ref INSTRUCTION_REGEX: Regex = Regex::new(r"^([NSEWLRF])([0-9]+)$").unwrap();
    static ref TEST_INPUT: Vec<Instruction> = vec!["F10", "N3", "F7", "R90", "F11",]
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

impl Direction {
    pub fn clockwise(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }

    pub fn counter_clockwise(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
        }
    }
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = INSTRUCTION_REGEX
            .captures(s)
            .ok_or(anyhow!("Didn't match regex: {}", s))?;

        let number: i32 = i32::from_str(&captures[2])?;

        let parsed = match &captures[1] {
            "N" => Instruction::Direction(Direction::North, number),
            "S" => Instruction::Direction(Direction::South, number),
            "E" => Instruction::Direction(Direction::East, number),
            "W" => Instruction::Direction(Direction::West, number),
            "L" => Instruction::Turn(-number / 90),
            "R" => Instruction::Turn(number / 90),
            "F" => Instruction::Forward(number),
            nope => panic!("Unrecognized instruction: {}", nope),
        };

        Ok(parsed)
    }
}
