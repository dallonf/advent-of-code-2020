// Day 24: Lobby Layout

use std::{borrow::Cow, collections::HashSet};

use rayon::prelude::*;
use shared::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tile(i64, i64, i64);

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TilePattern(HashSet<Tile>);

lazy_static! {
    static ref TEST_INPUT: Vec<&'static str> = puzzle_input::lines(include_str!("test_input.txt"));
    static ref PUZZLE_INPUT: Vec<&'static str> =
        puzzle_input::lines(include_str!("puzzle_input.txt"));
}

impl Direction {
    pub fn all() -> Vec<Self> {
        vec![
            Direction::E,
            Direction::SE,
            Direction::SW,
            Direction::W,
            Direction::NE,
            Direction::NW,
        ]
    }
}

impl Tile {
    pub fn from_directions_str(directions: &str) -> anyhow::Result<Tile> {
        let mut iter = directions.chars();

        let mut tile = Tile::default();
        loop {
            let next_direction = match iter.next() {
                Some('e') => Ok(Direction::E),
                Some('s') => match iter.next() {
                    Some('e') => Ok(Direction::SE),
                    Some('w') => Ok(Direction::SW),
                    _ => Err(anyhow!("expected e or w")),
                },
                Some('w') => Ok(Direction::W),
                Some('n') => match iter.next() {
                    Some('e') => Ok(Direction::NE),
                    Some('w') => Ok(Direction::NW),
                    _ => Err(anyhow!("expected e or w")),
                },
                Some(_) => Err(anyhow!("unrecognized character")),
                None => break,
            }?;
            tile = tile.neighbor(next_direction);
        }

        Ok(tile)
    }

    pub fn neighbor(&self, direction: Direction) -> Tile {
        let (dx, dy, dz) = match direction {
            Direction::E => (1, -1, 0),
            Direction::SE => (0, -1, 1),
            Direction::SW => (-1, 0, 1),
            Direction::W => (-1, 1, 0),
            Direction::NW => (0, 1, -1),
            Direction::NE => (1, 0, -1),
        };

        let &Tile(x, y, z) = self;

        Tile(x + dx, y + dy, z + dz)
    }

    pub fn all_neighbors(&self) -> impl Iterator<Item = Tile> + '_ {
        Direction::all()
            .into_iter()
            .map(move |direction| self.neighbor(direction))
    }
}

impl TilePattern {
    pub fn from_instructions(instructions: &[&str]) -> anyhow::Result<Self> {
        let tiles: anyhow::Result<HashSet<Tile>> =
            instructions
                .iter()
                .try_fold(HashSet::new(), |mut tiles, &instruction| {
                    let tile = Tile::from_directions_str(instruction)?;
                    if tiles.contains(&tile) {
                        tiles.remove(&tile);
                    } else {
                        tiles.insert(tile);
                    }
                    Ok(tiles)
                });

        Ok(TilePattern(tiles?))
    }

    pub fn count_black_tiles(&self) -> usize {
        self.0.len()
    }

    pub fn update(&self) -> Self {
        let tiles_to_consider: HashSet<Tile> = self
            .0
            .iter()
            .flat_map(|x| x.all_neighbors().chain(std::iter::once(*x)))
            .collect();

        let tiles: HashSet<Tile> = tiles_to_consider
            .into_par_iter()
            .filter(|tile| {
                let is_black = self.0.contains(&tile);
                let black_neighbors = tile
                    .all_neighbors()
                    .filter(|neighbor| self.0.contains(neighbor))
                    .count();
                if is_black {
                    !(black_neighbors == 0 || black_neighbors > 2)
                } else {
                    black_neighbors == 2
                }
            })
            .collect();

        TilePattern(tiles)
    }

    pub fn update_for_days(&self, days: usize) -> Self {
        (0..days)
            .into_iter()
            .fold(Cow::Borrowed(self), |pattern, _| {
                Cow::Owned(pattern.update())
            })
            .into_owned()
    }
}

pub fn part_one(instructions: &[&str]) -> anyhow::Result<usize> {
    Ok(TilePattern::from_instructions(instructions)?.count_black_tiles())
}

#[cfg(test)]
mod part_one {
    use super::*;

    #[test]
    fn test_directions() {
        assert_ne!(Tile::from_directions_str("esew").unwrap(), Tile::default());
        assert_eq!(
            Tile::from_directions_str("nwwswee").unwrap(),
            Tile::default()
        );
    }

    #[test]
    fn test_case() {
        assert_eq!(part_one(TEST_INPUT.as_slice()).unwrap(), 10);
    }

    #[test]
    fn answer() {
        assert_eq!(part_one(PUZZLE_INPUT.as_slice()).unwrap(), 495);
    }
}

#[cfg(test)]
mod part_two {
    use super::*;

    #[test]
    fn test_updates() {
        let floor = TilePattern::from_instructions(TEST_INPUT.as_slice()).unwrap();
        let floor = floor.update();
        assert_eq!(floor.count_black_tiles(), 15);
        let floor = floor.update();
        assert_eq!(floor.count_black_tiles(), 12);
        let floor = floor.update();
        assert_eq!(floor.count_black_tiles(), 25);
    }
    #[test]
    fn test_case() {
        assert_eq!(
            TilePattern::from_instructions(TEST_INPUT.as_slice())
                .unwrap()
                .update_for_days(100)
                .count_black_tiles(),
            2208
        );
    }

    #[test]
    fn answer() {
        assert_eq!(
            TilePattern::from_instructions(PUZZLE_INPUT.as_slice())
                .unwrap()
                .update_for_days(100)
                .count_black_tiles(),
            4012
        );
    }
}
