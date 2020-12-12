use crate::Direction;
use crate::Instruction;

#[derive(Copy, Clone)]
pub struct ShipState {
    direction: Direction,
    x: i32,
    y: i32,
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
mod test {
    use super::*;
    use crate::PUZZLE_INPUT;
    use crate::TEST_INPUT;

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
