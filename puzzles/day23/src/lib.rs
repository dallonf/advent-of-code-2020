// Day 23: Crab Cups

use std::{collections::HashMap, convert::TryInto, str::FromStr};

use shared::prelude::*;

const TEST_INPUT: &str = "389125467";
const PUZZLE_INPUT: &str = "463528179";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrabGame {
    highest_label: u8,
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
            next_cup_map,
        })
    }
}

impl CrabGame {
    pub fn perform_move(self) -> CrabGame {
        todo!();
    }

    pub fn perform_moves(self, moves: usize) -> CrabGame {
        todo!()
    }

    pub fn cups_after_1(&self) -> CrabGameIterator {
        CrabGameIterator {
            game: self,
            current: *self.next_cup_map.get(&1).unwrap(),
            halt_at: Some(1),
        }
    }

    pub fn output_string(&self) -> String {
        self.cups_after_1()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("")
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
            self.current = *self.game.next_cup_map.get(&self.current).unwrap();
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
