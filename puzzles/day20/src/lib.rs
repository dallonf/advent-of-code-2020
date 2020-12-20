// Day 20: Jurassic Jigsaw

use shared::prelude::*;

#[derive(Debug)]
pub struct Tile {
    tile_id: u64,
    size: usize,
    data: Vec<bool>,
}

lazy_static! {
    static ref TILE_REGEX: Regex = Regex::new(r"^Tile ([0-9]+):$").unwrap();
    static ref TEST_INPUT: Vec<Tile> =
        parse_input(&puzzle_input::lines(include_str!("test_input.txt"))).unwrap();
    static ref PUZZLE_INPUT: Vec<Tile> =
        parse_input(&puzzle_input::lines(include_str!("puzzle_input.txt"))).unwrap();
}

pub fn parse_input(input: &[&str]) -> anyhow::Result<Vec<Tile>> {
    input
        .split(|line| line.is_empty())
        .map(|section| Tile::parse(section))
        .collect()
}

impl Tile {
    pub fn parse(section: &[&str]) -> anyhow::Result<Tile> {
        let first_line = section.get(0).ok_or(anyhow!("empty section"))?;
        let tile_id = TILE_REGEX
            .captures(first_line)
            .ok_or(anyhow!("invalid header line"))
            .and_then(|captures| captures[1].parse::<u64>().map_err(|err| err.into()))?;

        let size = section.len() - 1;

        if !section.iter().all(|line| line.len() == size) {
            return Err(anyhow!("Tile isn't square"));
        }

        let data = section
            .iter()
            .flat_map(|line| line.chars().map(|char| char == '#'))
            .collect();

        Ok(Tile {
            tile_id,
            size,
            data,
        })
    }
}

#[cfg(test)]
mod part_one {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(1 + 1, 2);
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
