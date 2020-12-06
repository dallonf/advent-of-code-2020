// Day 5: Binary Boarding

use core::fmt::Debug;
use shared::prelude::*;
use std::error::Error;
use std::str::FromStr;

pub struct BoardingPassSeat {
    row: u8,
    column: u8,
}

lazy_static! {
    static ref PUZZLE_INPUT: Vec<BoardingPassSeat> =
        puzzle_input::lines(include_str!("puzzle_input.txt"))
            .into_iter()
            .map(BoardingPassSeat::from_str)
            .collect::<Result<_, _>>()
            .unwrap();
}

impl BoardingPassSeat {
    pub fn seat_id(&self) -> u32 {
        (self.row as u32) * 8 + (self.column as u32)
    }
}

impl Debug for BoardingPassSeat {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        Ok(println!(
            "row {}, column {}, seat ID {}",
            self.row,
            self.column,
            self.seat_id()
        ))
    }
}

impl FromStr for BoardingPassSeat {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
        let row = input
            .chars()
            .take(7)
            .try_fold(
                (0 as u16, 127 as u16),
                |(lower_bound, upper_bound), next| {
                    let midpoint = lower_bound + (upper_bound - lower_bound) / 2;
                    match next {
                        'F' => Ok((lower_bound, midpoint)),
                        'B' => Ok((midpoint + 1, upper_bound)),
                        _ => Err(format!("Unexpected char: {}", next)),
                    }
                },
            )?
            .0 as u8;

        let column = input
            .chars()
            .skip(7)
            .try_fold((0, 7), |(lower_bound, upper_bound), next| {
                let midpoint = lower_bound + (upper_bound - lower_bound) / 2;
                match next {
                    'L' => Ok((lower_bound, midpoint)),
                    'R' => Ok((midpoint + 1, upper_bound)),
                    _ => Err(format!("Unexpected char: {}", next)),
                }
            })?
            .0;

        Ok(BoardingPassSeat { row, column })
    }
}

pub fn find_missing_seat(seats: &[BoardingPassSeat]) -> Result<u32, Box<dyn Error>> {
    let mut ids: Vec<u32> = seats.iter().map(BoardingPassSeat::seat_id).collect();
    ids.sort();

    ids.iter()
        .copied()
        .scan(None, |prev_id, id| {
            let result = (prev_id.clone(), id);
            *prev_id = Some(id);
            Some(result)
        })
        // locate the ID that breaks the pattern of prev_id = id - 1 and is actually 2 away, implying one
        // was skipped in the sequence
        .find(|(prev_id, id)| {
            println!("🥐 {:?}, {}", prev_id, id);
            prev_id.map_or(false, |prev_id| *id == prev_id + 2)
        })
        .map_or(Err("Couldn't find a missing ID".into()), |(_, id)| {
            Ok(id - 1)
        })
}

#[cfg(test)]
mod part_one {
    use super::*;

    #[test]
    fn test_parse() {
        let pass = BoardingPassSeat::from_str("FBFBBFFRLR").unwrap();
        assert_eq!(pass.row, 44);
        assert_eq!(pass.column, 5);
        assert_eq!(pass.seat_id(), 357);
    }

    #[test]
    fn test_cases() {
        let pass = BoardingPassSeat::from_str("BFFFBBFRRR").unwrap();
        assert_eq!(pass.row, 70);
        assert_eq!(pass.column, 7);
        assert_eq!(pass.seat_id(), 567);

        let pass = BoardingPassSeat::from_str("FFFBBBFRRR").unwrap();
        assert_eq!(pass.row, 14);
        assert_eq!(pass.column, 7);
        assert_eq!(pass.seat_id(), 119);

        let pass = BoardingPassSeat::from_str("BBFFBBFRLL").unwrap();
        assert_eq!(pass.row, 102);
        assert_eq!(pass.column, 4);
        assert_eq!(pass.seat_id(), 820);
    }

    #[test]
    fn answer() {
        assert_eq!(
            PUZZLE_INPUT
                .iter()
                .map(BoardingPassSeat::seat_id)
                .max()
                .unwrap(),
            926
        );
    }
}

#[cfg(test)]
mod part_two {
    use super::*;

    #[test]
    fn answer() {
        let result = find_missing_seat(&PUZZLE_INPUT).unwrap();
        assert!(result > 383); // found a wrong answer
        assert_eq!(result, 657);
    }
}
