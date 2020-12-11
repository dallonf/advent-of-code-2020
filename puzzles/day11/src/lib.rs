// Day 11: Seating System

use std::{borrow::Cow, str::FromStr};

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
    // static ref PUZZLE_INPUT: &'static str = include_str!("puzzle_input.txt");

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
    fn from_list(row_len: usize, seats: Cow<Vec<SeatLayout>>) -> SeatLayout {
        todo!()
    }

    fn items_iter(&self) -> impl Iterator<Item = (usize, usize, &SeatLayout)> {
        todo!();
        vec![].into_iter()
    }

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

    fn seat_at(&self, coordinate: &(usize, usize)) -> &SeatState {
        todo!()
    }

    fn adjacent_seats(&self, coordinate: &(usize, usize)) -> impl Iterator<Item = &SeatState> {
        todo!();
        vec![].into_iter()
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
