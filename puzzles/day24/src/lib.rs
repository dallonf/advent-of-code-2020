// Day 24: Lobby Layout

use shared::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Tile(i64, i64, i64);

lazy_static! {
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

    // #[test]
    // fn test_cases() {
    //     assert_eq!(1 + 1, 2);
    // }

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
