use std::collections::{HashSet, VecDeque};

// Day 22: Crab Combat
use shared::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    Player1,
    Player2,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeckState {
    player1_cards: VecDeque<u32>,
    player2_cards: VecDeque<u32>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
    deck_state: DeckState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecursiveGameState {
    prev_states: HashSet<DeckState>,
    deck_state: DeckState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoundResult<T> {
    Active { winner: Player, game_state: T },
    Finished(GameResult),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameResult {
    winner: Player,
    winning_deck: Vec<u32>,
}

lazy_static! {
    static ref TEST_INPUT: Vec<&'static str> = puzzle_input::lines(include_str!("test_input.txt"));
    static ref PUZZLE_INPUT: Vec<&'static str> =
        puzzle_input::lines(include_str!("puzzle_input.txt"));
}

impl GameResult {
    pub fn new(deck_state: &DeckState, winner: Player) -> Self {
        let winning_deck = match winner {
            Player::Player1 => &deck_state.player1_cards,
            Player::Player2 => &deck_state.player2_cards,
        };

        Self {
            winner,
            winning_deck: winning_deck.iter().copied().collect::<Vec<u32>>(),
        }
    }
}

impl DeckState {
    pub fn parse(input: &[&str]) -> anyhow::Result<Self> {
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

        Ok(DeckState {
            player1_cards: parse_section(sections[0])?,
            player2_cards: parse_section(sections[1])?,
        })
    }
}

impl GameState {
    pub fn parse(input: &[&str]) -> anyhow::Result<Self> {
        Ok(GameState {
            deck_state: DeckState::parse(input)?,
        })
    }

    pub fn round(mut self) -> RoundResult<Self> {
        if self.deck_state.player1_cards.len() == 0 {
            RoundResult::Finished(GameResult::new(&self.deck_state, Player::Player2))
        } else if self.deck_state.player2_cards.len() == 0 {
            RoundResult::Finished(GameResult::new(&self.deck_state, Player::Player1))
        } else {
            let player1_card = self.deck_state.player1_cards.pop_front().unwrap();
            let player2_card = self.deck_state.player2_cards.pop_front().unwrap();
            let winner;

            if player1_card > player2_card {
                winner = Player::Player1;
                self.deck_state.player1_cards.push_back(player1_card);
                self.deck_state.player1_cards.push_back(player2_card);
            } else {
                winner = Player::Player2;
                self.deck_state.player2_cards.push_back(player2_card);
                self.deck_state.player2_cards.push_back(player1_card);
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

impl RecursiveGameState {
    pub fn parse(input: &[&str]) -> anyhow::Result<Self> {
        Ok(RecursiveGameState {
            prev_states: HashSet::new(),
            deck_state: DeckState::parse(input)?,
        })
    }

    pub fn round(mut self) -> RoundResult<Self> {
        let mut deck_state = self.deck_state;

        if self.prev_states.contains(&deck_state) {
            // infinite loop prevention
            RoundResult::Finished(GameResult::new(&deck_state, Player::Player1))
        } else if deck_state.player1_cards.len() == 0 {
            RoundResult::Finished(GameResult::new(&deck_state, Player::Player2))
        } else if deck_state.player2_cards.len() == 0 {
            RoundResult::Finished(GameResult::new(&deck_state, Player::Player1))
        } else {
            self.prev_states.insert(deck_state.clone());

            let player1_card = deck_state.player1_cards.pop_front().unwrap();
            let player2_card = deck_state.player2_cards.pop_front().unwrap();

            let winner = if deck_state.player1_cards.len() >= player1_card as usize
                && deck_state.player2_cards.len() >= player2_card as usize
            {
                let new_deck = DeckState {
                    player1_cards: deck_state
                        .player1_cards
                        .iter()
                        .take(player1_card as usize)
                        .copied()
                        .collect(),
                    player2_cards: deck_state
                        .player2_cards
                        .iter()
                        .take(player2_card as usize)
                        .copied()
                        .collect(),
                };

                let subgame = RecursiveGameState {
                    deck_state: new_deck,
                    prev_states: HashSet::new(),
                };

                subgame.game_result().winner
            } else if player1_card > player2_card {
                Player::Player1
            } else {
                Player::Player2
            };

            match winner {
                Player::Player1 => {
                    deck_state.player1_cards.push_back(player1_card);
                    deck_state.player1_cards.push_back(player2_card);
                }
                Player::Player2 => {
                    deck_state.player2_cards.push_back(player2_card);
                    deck_state.player2_cards.push_back(player1_card);
                }
            };

            self.deck_state = deck_state;
            RoundResult::Active {
                game_state: self,
                winner,
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
    Ok(score_deck(&winning_deck))
}

pub fn part_two(input: &[&str]) -> anyhow::Result<u32> {
    let game_state = RecursiveGameState::parse(input)?;
    let GameResult { winning_deck, .. } = game_state.game_result();
    Ok(score_deck(&winning_deck))
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
                    deck_state: DeckState {
                        player1_cards: vec![2, 6, 3, 1, 9, 5].into_iter().collect(),
                        player2_cards: vec![8, 4, 7, 10].into_iter().collect(),
                    }
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

#[cfg(test)]
mod part_two {
    use super::*;

    #[test]
    fn test_infinite_loop_prevention() {
        let game_state = RecursiveGameState::parse(&vec![
            "Player 1:",
            "43",
            "19",
            "",
            "Player 2:",
            "2",
            "29",
            "14",
        ])
        .unwrap();

        let GameResult { winner, .. } = game_state.game_result();

        assert_eq!(winner, Player::Player1);
    }

    #[test]
    fn test_case() {
        assert_eq!(part_two(TEST_INPUT.as_slice()).unwrap(), 291);
    }

    #[test]
    fn answer() {
        assert_eq!(part_two(PUZZLE_INPUT.as_slice()).unwrap(), 33212);
    }
}
