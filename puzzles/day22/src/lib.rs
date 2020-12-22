use std::collections::VecDeque;

// Day 22: Crab Combat
use shared::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    Player1,
    Player2,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
    player1_cards: VecDeque<u32>,
    player2_cards: VecDeque<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoundResult {
    Active {
        winner: Player,
        game_state: GameState,
    },
    Finished(GameResult),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameResult {
    winner: Player,
    winning_deck: VecDeque<u32>,
}

lazy_static! {
    static ref TEST_INPUT: Vec<&'static str> = puzzle_input::lines(include_str!("test_input.txt"));
    static ref PUZZLE_INPUT: Vec<&'static str> =
        puzzle_input::lines(include_str!("puzzle_input.txt"));
}

impl GameState {
    pub fn parse(input: &[&str]) -> anyhow::Result<GameState> {
        let sections: Vec<&[&str]> = input.split(|line| line.is_empty()).collect();

        if sections.len() != 2 {
            return Err(anyhow!("Wrong number of sections"));
        }
        if sections[0][0] != "Player 1:" {
            return Err(anyhow!("Player 1 section is missing"));
        }
        if sections[1][0] != "Player 2:" {
            return Err(anyhow!("Player 2 section is missing"));
        }

        fn parse_section(section: &[&str]) -> anyhow::Result<VecDeque<u32>> {
            section
                .iter()
                .skip(1)
                .map(|line| line.parse().map_err(anyhow::Error::from))
                .collect()
        }

        Ok(GameState {
            player1_cards: parse_section(sections[0])?,
            player2_cards: parse_section(sections[1])?,
        })
    }

    pub fn round(mut self) -> RoundResult {
        if self.player1_cards.len() == 0 || self.player2_cards.len() == 0 {
            // return the results of the last round
            RoundResult::Finished(if self.player1_cards.len() == 0 {
                GameResult {
                    winner: Player::Player2,
                    winning_deck: self.player2_cards.to_owned(),
                }
            } else {
                GameResult {
                    winner: Player::Player1,
                    winning_deck: self.player1_cards.to_owned(),
                }
            })
        } else {
            let player1_card = self.player1_cards.pop_front().unwrap();
            let player2_card = self.player2_cards.pop_front().unwrap();
            let winner;

            if player1_card > player2_card {
                winner = Player::Player1;
                self.player1_cards.push_back(player1_card);
                self.player1_cards.push_back(player2_card);
            } else {
                winner = Player::Player2;
                self.player2_cards.push_back(player2_card);
                self.player2_cards.push_back(player1_card);
            }

            RoundResult::Active {
                winner,
                game_state: self,
            }
        }
    }

    pub fn game_result(mut self) -> GameResult {
        loop {
            let round_result = self.round();
            match round_result {
                RoundResult::Active {
                    winner: _,
                    game_state,
                } => self = game_state,
                RoundResult::Finished(result) => return result,
            }
        }
    }
}

pub fn score_deck(deck: &[u32]) -> u32 {
    let cards_in_deck = deck.len() as u32;
    deck.iter()
        .enumerate()
        .map(|(i, card)| card * (cards_in_deck - i as u32))
        .sum()
}

pub fn part_one(input: &[&str]) -> anyhow::Result<u32> {
    let game_state = GameState::parse(input)?;
    let GameResult { winning_deck, .. } = game_state.game_result();
    Ok(score_deck(
        &winning_deck.iter().copied().collect::<Vec<u32>>(),
    ))
}

#[cfg(test)]
mod part_one {
    use super::*;

    #[test]
    fn test_round() {
        let game_state = GameState::parse(TEST_INPUT.as_slice()).unwrap();
        let game_state = game_state.round();
        assert_eq!(
            game_state,
            RoundResult::Active {
                winner: Player::Player1,
                game_state: GameState {
                    player1_cards: vec![2, 6, 3, 1, 9, 5].into_iter().collect(),
                    player2_cards: vec![8, 4, 7, 10].into_iter().collect(),
                }
            }
        );
    }

    #[test]
    fn test_case() {
        assert_eq!(part_one(TEST_INPUT.as_slice()).unwrap(), 306);
    }

    #[test]
    fn answer() {
        assert_eq!(part_one(PUZZLE_INPUT.as_slice()).unwrap(), 31957);
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
