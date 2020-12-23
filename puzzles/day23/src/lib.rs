// Day 23: Crab Cups

use std::{collections::HashMap, str::FromStr};

use shared::prelude::*;

const TEST_INPUT: &str = "389125467";
const PUZZLE_INPUT: &str = "463528179";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrabGame {
    highest_label: u8,
    lowest_label: u8,
    pub current_cup: u8,
    next_cup_map: HashMap<u8, u8>,
}

impl FromStr for CrabGame {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<u8> = s
            .chars()
            .map(|char| char.to_string().parse())
            .collect::<Result<_, _>>()?;

        if numbers.len() <= 4 {
            return Err(anyhow!("Sequence requires at least 4 numbers"));
        }

        let current_cup = numbers[0];
        let highest_label = *numbers.iter().max().unwrap();
        let lowest_label = *numbers.iter().min().unwrap();

        let next_cup_map = numbers
            .windows(2)
            .map(|window| (window[0], window[1]))
            .chain(std::iter::once((
                *numbers.last().unwrap(),
                *numbers.first().unwrap(),
            )))
            .collect();

        Ok(CrabGame {
            current_cup,
            highest_label,
            lowest_label,
            next_cup_map,
        })
    }
}

impl CrabGame {
    pub fn perform_move(mut self) -> CrabGame {
        let picked_up_cups: Vec<u8> = CrabGameIterator {
            game: &self,
            current: self.next_cup(self.current_cup).unwrap(),
            halt_at: None,
        }
        .take(3)
        .collect();

        // remove picked up cups from circle
        self.next_cup_map.insert(
            self.current_cup,
            self.next_cup(*picked_up_cups.last().unwrap()).unwrap(),
        );

        let destination = {
            let mut destination = self.current_cup;

            loop {
                destination -= 1;
                if destination < self.lowest_label {
                    destination = self.highest_label
                }

                if !picked_up_cups.contains(&destination) {
                    break;
                }
            }

            destination
        };

        // insert after destination
        let after_destination = self.next_cup(destination).unwrap();
        self.next_cup_map
            .insert(destination, *picked_up_cups.first().unwrap());
        self.next_cup_map
            .insert(*picked_up_cups.last().unwrap(), after_destination);

        self.current_cup = self.next_cup(self.current_cup).unwrap();

        self
    }

    pub fn perform_moves(self, moves: usize) -> CrabGame {
        (0..moves).fold(self, |game, _| game.perform_move())
    }

    pub fn cups_after_1(&self) -> CrabGameIterator {
        CrabGameIterator {
            game: self,
            current: self.next_cup(1).unwrap(),
            halt_at: Some(1),
        }
    }

    pub fn output_string(&self) -> String {
        self.cups_after_1()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn next_cup(&self, cup: u8) -> Option<u8> {
        self.next_cup_map.get(&cup).copied()
    }
}

pub struct CrabGameIterator<'a> {
    game: &'a CrabGame,
    current: u8,
    halt_at: Option<u8>,
}

impl Iterator for CrabGameIterator<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.halt_at == Some(self.current) {
            None
        } else {
            let result = self.current;
            self.current = self.game.next_cup(self.current).unwrap();
            Some(result)
        }
    }
}

#[cfg(test)]
mod part_one {
    use super::*;

    #[test]
    fn test_parse() {
        let game = CrabGame::from_str(TEST_INPUT).unwrap();
        assert_eq!(game.current_cup, 3);
        assert_eq!(game.highest_label, 9);
        assert_eq!(game.output_string(), "25467389");
    }

    #[test]
    fn test_move() {
        let game = CrabGame::from_str(TEST_INPUT).unwrap();
        let game = game.perform_move();
        assert_eq!(game.current_cup, 2);
        assert_eq!(game.output_string(), "54673289");
    }

    #[test]
    fn test_case() {
        let game = CrabGame::from_str(TEST_INPUT).unwrap();
        let game = game.perform_moves(10);
        assert_eq!(game.output_string(), "92658374");
        let game = game.perform_moves(90);
        assert_eq!(game.output_string(), "67384529");
    }

    #[test]
    fn answer() {
        let game = CrabGame::from_str(PUZZLE_INPUT).unwrap();
        let game = game.perform_moves(100);
        assert_eq!(game.output_string(), "52937846");
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
