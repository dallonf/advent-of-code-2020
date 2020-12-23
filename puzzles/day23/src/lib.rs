// Day 23: Crab Cups

use std::{collections::HashMap, str::FromStr};

use shared::prelude::*;

pub const TEST_INPUT: &str = "389125467";
pub const PUZZLE_INPUT: &str = "463528179";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrabGame {
    highest_label: u32,
    lowest_label: u32,
    pub current_cup: u32,
    next_cup_map: HashMap<u32, u32>,
}

impl FromStr for CrabGame {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<u32> = s
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
        let picked_up_cups: Vec<u32> = CrabGameIterator {
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

    pub fn next_cup(&self, cup: u32) -> Option<u32> {
        self.next_cup_map.get(&cup).copied()
    }

    pub fn prev_cup(&self, cup: u32) -> Option<u32> {
        self.next_cup_map
            .iter()
            .find_map(|(&prev, &current)| if current == cup { Some(prev) } else { None })
    }

    pub fn expand(mut self) -> Self {
        let mut prev_label = self.prev_cup(self.current_cup).unwrap();
        for additional_label in self.highest_label + 1..1_000_001 {
            self.next_cup_map.insert(prev_label, additional_label);
            prev_label = additional_label;
        }
        self.next_cup_map.insert(1_000_000, self.current_cup);
        self
    }

    pub fn output_mk2(&self) -> u64 {
        self.cups_after_1().take(2).map(|i| i as u64).product()
    }
}

pub struct CrabGameIterator<'a> {
    game: &'a CrabGame,
    current: u32,
    halt_at: Option<u32>,
}

impl Iterator for CrabGameIterator<'_> {
    type Item = u32;

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

#[cfg(test)]
mod part_two {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_expand() {
        let game = CrabGame::from_str(TEST_INPUT).unwrap().expand();
        assert_eq!(game.next_cup_map.len(), 1_000_000);
        assert_eq!(game.cups_after_1().collect::<HashSet<u32>>().len(), 999_999);
    }

    #[test]
    fn test_case() {
        let game = CrabGame::from_str(TEST_INPUT).unwrap().expand();
        let game = game.perform_moves(10_000_000);
        assert_eq!(
            game.cups_after_1().take(2).collect::<Vec<u32>>(),
            vec![934001, 159792]
        );
        assert_eq!(game.output_mk2(), 149245887792);
    }

    // #[test]
    // fn answer() {}
}
