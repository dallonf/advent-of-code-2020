// Day 17: Conway Cubes

use std::collections::HashSet;

use shared::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point(i32, i32, i32);

#[derive(Debug, Clone)]
pub struct ActiveCubes(HashSet<Point>);

lazy_static! {
    static ref TEST_INPUT: Vec<&'static str> = vec![".#.", "..#", "###,"];
    static ref PUZZLE_INPUT: Vec<&'static str> =
        puzzle_input::lines(include_str!("puzzle_input.txt"));
}

impl ActiveCubes {
    pub fn parse(s: &[&str]) -> ActiveCubes {
        let cubes = s
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .filter(|(_, char)| *char == '#')
                    .map(move |(x, _)| Point(x as i32, y as i32, 0))
            })
            .collect();

        ActiveCubes(cubes)
    }

    pub fn count(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod part_one {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(ActiveCubes::parse(TEST_INPUT.as_slice()).count(), 5);
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
