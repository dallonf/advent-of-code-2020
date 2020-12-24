// Day 24: Lobby Layout

use std::collections::HashSet;

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

// #[cfg(test)]
// mod part_two {
//     use super::*;
//     #[test]
//     fn test_cases() {}
//     #[test]
//     fn answer() {}
// }
