// Day 12: Rain Risk

use std::str::FromStr;

use shared::prelude::*;

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

#[derive(Copy, Clone)]
pub struct ShipState {
    direction: Direction,
    x: i32,
    y: i32,
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

impl Default for ShipState {
    fn default() -> Self {
        ShipState {
            direction: Direction::East,
            x: 0,
            y: 0,
        }
    }
}

impl ShipState {
    pub fn follow_instructions(&self, instructions: &[Instruction]) -> ShipState {
        instructions
            .into_iter()
            .fold(self.to_owned(), |state, instruction| {
                state.follow_instruction(instruction)
            })
    }

    pub fn follow_instruction(&self, instruction: &Instruction) -> ShipState {
        match instruction {
            Instruction::Direction(direction, n) => self.move_in_direction(*direction, *n),
            Instruction::Turn(n) => self.turn(*n),
            Instruction::Forward(n) => self.move_in_direction(self.direction, *n),
        }
    }

    pub fn move_in_direction(&self, direction: Direction, n: i32) -> ShipState {
        let (x, y) = match direction {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        };

        ShipState {
            x: self.x + x * n,
            y: self.y + y * n,
            ..*self
        }
    }

    pub fn turn(&self, turns: i32) -> ShipState {
        let turn = match turns > 0 {
            true => Direction::clockwise,
            false => Direction::counter_clockwise,
        };

        let new_direction = (0..turns.abs()).fold(self.direction, |prev, _| turn(&prev));

        ShipState {
            direction: new_direction,
            ..*self
        }
    }
}

pub fn manhattan_distance_of_instructions(instructions: &[Instruction]) -> i32 {
    let end_state = ShipState::default().follow_instructions(instructions);
    end_state.x.abs() + end_state.y.abs()
}

#[cfg(test)]
mod part_one {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            manhattan_distance_of_instructions(TEST_INPUT.as_slice()),
            25
        );
    }

    #[test]
    fn answer() {
        assert_eq!(
            manhattan_distance_of_instructions(PUZZLE_INPUT.as_slice()),
            1589
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
