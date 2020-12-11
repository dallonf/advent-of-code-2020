// Day 11: Seating System

use std::str::FromStr;

use shared::prelude::*;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum SeatState {
    Floor,
    Empty,
    Occupied,
}

#[derive(Eq, PartialEq, Clone)]
pub struct SeatLayout {
    seats: Vec<SeatState>,
    row_len: usize,
}

lazy_static! {
    static ref TEST_INPUT: SeatLayout = include_str!("test_input.txt").parse().unwrap();
    static ref PUZZLE_INPUT: SeatLayout = include_str!("puzzle_input.txt").parse().unwrap();
}

impl FromStr for SeatLayout {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = puzzle_input::lines(s);

        let rows: Vec<Vec<SeatState>> = lines
            .into_iter()
            .map(|l| {
                l.chars()
                    .map(|char| match char {
                        '.' => Ok(SeatState::Floor),
                        'L' => Ok(SeatState::Empty),
                        '#' => Ok(SeatState::Occupied),
                        other => Err(anyhow!("Unrecognized char code: {}", other)),
                    })
                    .collect::<Result<_, _>>()
            })
            .collect::<anyhow::Result<_>>()?;

        let row_len = rows[0].len();
        let seats: Vec<SeatState> = rows.iter().flatten().cloned().collect();

        Ok(SeatLayout { seats, row_len })
    }
}

impl SeatLayout {
    fn map(&self, f: impl Fn(&(usize, usize), &SeatState) -> SeatState + Copy) -> SeatLayout {
        let new_seats = (0..(self.seats.len() / self.row_len)).flat_map(move |y| {
            (0..self.row_len).map(move |x| {
                let coordinate = (x, y);
                let seat = self.seat_at(&coordinate);
                f(&coordinate, seat)
            })
        });

        SeatLayout {
            row_len: self.row_len,
            seats: new_seats.collect(),
        }
    }

    fn seat_at(&self, (x, y): &(usize, usize)) -> &SeatState {
        let index = y * self.row_len + x;
        return &self.seats[index];
    }

    fn adjacent_seats(&self, (x, y): &(usize, usize)) -> impl Iterator<Item = &SeatState> {
        let directions = {
            let max_x = self.row_len - 1;
            let max_y = (self.seats.len() / self.row_len) - 1;
            let x_left = if x <= &0 { None } else { Some(x - 1_usize) };
            let x_right = if x >= &max_x { None } else { Some(x + 1_usize) };
            let y_up = if y <= &0 { None } else { Some(y - 1_usize) };
            let y_down = if y >= &max_y { None } else { Some(y + 1_usize) };
            let x = Some(*x);
            let y = Some(*y);
            vec![
                (x, y_up),         // up
                (x_right, y_up),   // up-right
                (x_right, y),      // right
                (x_right, y_down), // down-right
                (x, y_down),       // down
                (x_left, y_down),  // down-left
                (x_left, y),       // left
                (x_left, y_up),    // up-left
            ]
        };

        directions.into_iter().filter_map(move |(x, y)| {
            if let (Some(x), Some(y)) = (x, y) {
                Some(self.seat_at(&(x, y)))
            } else {
                None
            }
        })
    }

    pub fn iterate(&self) -> SeatLayout {
        self.map(|coord, seat| {
            if seat == &SeatState::Floor {
                return *seat;
            }

            let adjacent_occupied = self
                .adjacent_seats(coord)
                .filter(|x| x == &&SeatState::Occupied)
                .count();

            if seat == &SeatState::Empty && adjacent_occupied == 0 {
                SeatState::Occupied
            } else if seat == &SeatState::Occupied && adjacent_occupied >= 4 {
                SeatState::Empty
            } else {
                *seat
            }
        })
    }

    pub fn iterate_until_stable(&self) -> SeatLayout {
        let next_state = self.iterate();
        if &next_state == self {
            next_state
        } else {
            next_state.iterate_until_stable()
        }
    }

    pub fn occupied(&self) -> usize {
        self.seats
            .iter()
            .filter(|x| x == &&SeatState::Occupied)
            .count()
    }
}

#[cfg(test)]
mod part_one {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(TEST_INPUT.iterate_until_stable().occupied(), 37);
    }

    #[test]
    fn answer() {
        assert_eq!(PUZZLE_INPUT.iterate_until_stable().occupied(), 2489);
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
