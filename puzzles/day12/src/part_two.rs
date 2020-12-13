use crate::Direction;
use crate::Instruction;
use core::ops::Add;
use core::ops::Mul;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Point(i32, i32);

#[derive(Debug, Copy, Clone)]
pub struct ShipState {
    position: Point,
    waypoint: Point,
}

impl Mul<i32> for Point {
    type Output = Point;
    fn mul(self, rhs: i32) -> Self::Output {
        Point(self.0 * rhs, self.1 * rhs)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Point {
    pub fn clockwise(self) -> Point {
        let Point(x, y) = self;
        Point(y * -1, x)
    }

    pub fn counter_clockwise(self) -> Point {
        let Point(x, y) = self;
        Point(y, x * -1)
    }

    pub fn turn(&self, turns: i32) -> Point {
        let turn = match turns > 0 {
            true => Point::clockwise,
            false => Point::counter_clockwise,
        };

        (0..turns.abs()).fold(self.to_owned(), |prev, _| turn(prev))
    }

    pub fn add_direction(&self, direction: Direction, n: i32) -> Point {
        let delta = match direction {
            Direction::North => Point(0, -1),
            Direction::South => Point(0, 1),
            Direction::East => Point(1, 0),
            Direction::West => Point(-1, 0),
        };

        *self + delta * n
    }
}

impl Default for ShipState {
    fn default() -> Self {
        ShipState {
            position: Point(0, 0),
            waypoint: Point(10, -1),
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
            Instruction::Direction(direction, n) => {
                let waypoint = self.waypoint.add_direction(*direction, *n);
                ShipState { waypoint, ..*self }
            }
            Instruction::Turn(n) => ShipState {
                waypoint: self.waypoint.turn(*n),
                ..*self
            },
            Instruction::Forward(n) => {
                let movement = self.waypoint * *n;
                ShipState {
                    position: self.position + movement,
                    ..*self
                }
            }
        }
    }
}

pub fn manhattan_distance_of_instructions(instructions: &[Instruction]) -> i32 {
    let end_state = ShipState::default().follow_instructions(instructions);
    let Point(x, y) = end_state.position;
    x.abs() + y.abs()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::PUZZLE_INPUT;
    use crate::TEST_INPUT;

    #[test]
    fn test_clockwise() {
        assert_eq!(
            vec![
                Point(0, 0),
                Point(0, -1),
                Point(-1, -1),
                Point(1, 0),
                Point(1, -1),
            ]
            .into_iter()
            .map(|x| x.clockwise())
            .collect::<Vec<Point>>(),
            vec![
                Point(0, 0),
                Point(1, 0),
                Point(1, -1),
                Point(0, 1),
                Point(1, 1)
            ]
        );
    }

    #[test]
    fn test_counter_clockwise() {
        assert_eq!(
            vec![
                Point(0, 0),
                Point(1, 0),
                Point(1, -1),
                Point(0, 1),
                Point(1, 1)
            ]
            .into_iter()
            .map(|x| x.counter_clockwise())
            .collect::<Vec<Point>>(),
            vec![
                Point(0, 0),
                Point(0, -1),
                Point(-1, -1),
                Point(1, 0),
                Point(1, -1),
            ]
        );
    }

    #[test]
    fn test_cases() {
        assert_eq!(
            manhattan_distance_of_instructions(TEST_INPUT.as_slice()),
            286
        );
    }

    #[test]
    fn answer() {
        assert_eq!(
            manhattan_distance_of_instructions(PUZZLE_INPUT.as_slice()),
            23960
        );
    }
}
