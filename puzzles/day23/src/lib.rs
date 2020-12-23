// Day 23: Crab Cups

use std::str::FromStr;

use shared::prelude::*;

pub const TEST_INPUT: &str = "389125467";
pub const PUZZLE_INPUT: &str = "463528179";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrabGame {
    highest_label: u32,
    lowest_label: u32,
    pub current_cup: u32,
    next_cup_map: Vec<u32>,
}

impl FromStr for CrabGame {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<u32> = s
            .chars()
            .map(|char| char.to_string().parse())
            .collect::<Result<_, _>>()?;

        CrabGame::from_list(&numbers)
    }
}

impl CrabGame {
    pub fn from_list(numbers: &[u32]) -> anyhow::Result<Self> {
        if numbers.len() <= 4 {
            return Err(anyhow!("Sequence requires at least 4 numbers"));
        }

        let current_cup = numbers[0];
        let highest_label = *numbers.iter().max().unwrap();
        let lowest_label = *numbers.iter().min().unwrap();

        // This actually leaves a 0 at the beginning of the list, but that's fine
        // less costly than subtracting every index by 1 on access
        let mut next_cup_map: Vec<u32> = std::iter::repeat(0).take(numbers.len() + 1).collect();
        for window in numbers.windows(2) {
            next_cup_map[window[0] as usize] = window[1];
        }
        next_cup_map[*numbers.last().unwrap() as usize] = *numbers.first().unwrap();

        Ok(CrabGame {
            current_cup,
            highest_label,
            lowest_label,
            next_cup_map,
        })
    }

    pub fn from_list_expanded(numbers: &[u32]) -> anyhow::Result<Self> {
        let mut numbers = numbers.to_vec();
        numbers.extend(numbers.len() as u32 + 1..1_000_001);
        Self::from_list(&numbers)
    }

    pub fn from_str_expanded(s: &str) -> anyhow::Result<Self> {
        let numbers: Vec<u32> = s
            .chars()
            .map(|char| char.to_string().parse())
            .collect::<Result<_, _>>()?;

        Self::from_list_expanded(&numbers)
    }

    pub fn perform_move(mut self) -> CrabGame {
        let pickup_first = self.next_cup(self.current_cup).unwrap();
        let pickup_middle = self.next_cup(pickup_first).unwrap();
        let pickup_last = self.next_cup(pickup_middle).unwrap();

        // remove picked up cups from circle
        self.next_cup_map[self.current_cup as usize] = self.next_cup(pickup_last).unwrap();

        let destination = {
            let mut destination = self.current_cup;

            loop {
                destination -= 1;
                if destination < self.lowest_label {
                    destination = self.highest_label
                }

                if destination != pickup_first
                    && destination != pickup_middle
                    && destination != pickup_last
                {
                    break;
                }
            }

            destination
        };

        // insert after destination
        let after_destination = self.next_cup(destination).unwrap();
        self.next_cup_map[destination as usize] = pickup_first;
        self.next_cup_map[pickup_last as usize] = after_destination;

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
        self.next_cup_map.get(cup as usize).copied()
    }

    pub fn prev_cup(&self, cup: u32) -> Option<u32> {
        self.next_cup_map
            .iter()
            .enumerate()
            .find_map(|(prev, &current)| {
                if current == cup {
                    Some(prev as u32)
                } else {
                    None
                }
            })
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
        let game = CrabGame::from_str_expanded(TEST_INPUT).unwrap();
        assert_eq!(game.cups_after_1().collect::<HashSet<u32>>().len(), 999_999);
        assert_eq!(game.highest_label, 1_000_000);
    }

    #[test]
    fn test_case() {
        let game = CrabGame::from_str_expanded(TEST_INPUT).unwrap();
        let game = game.perform_moves(10_000_000);
        assert_eq!(
            game.cups_after_1().take(2).collect::<Vec<u32>>(),
            vec![934001, 159792]
        );
        assert_eq!(game.output_mk2(), 149245887792);
    }

    #[test]
    fn answer() {
        let game = CrabGame::from_str_expanded(PUZZLE_INPUT).unwrap();
        let game = game.perform_moves(10_000_000);
        assert_eq!(game.output_mk2(), 8456532414);
    }
}
