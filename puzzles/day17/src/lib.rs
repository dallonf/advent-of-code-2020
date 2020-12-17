// Day 17: Conway Cubes

use std::{collections::HashSet, ops::Add};

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

impl Point {
    pub fn neighbors(&self) -> impl Iterator<Item = Point> + '_ {
        let directions = (-1..2)
            .flat_map(|x| (-1..2).flat_map(move |y| (-1..2).map(move |z| Point(x, y, z))))
            .filter(|x| x != &Point(0, 0, 0)); // make sure 0,0 isn't in the list of directions!!

        directions.map(move |direction| *self + direction)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
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

    pub fn cycle(&self) -> ActiveCubes {
        // All active cubes plus all their neighbors
        let search_space: HashSet<Point> = self
            .0
            .iter()
            .flat_map(|point| point.neighbors())
            .chain(self.0.iter().copied())
            .collect();

        let active_neighbors = |point: &Point| -> usize {
            point
                .neighbors()
                .filter(|neighbor| self.0.contains(neighbor))
                .count()
        };

        let next_active: HashSet<Point> = search_space
            .iter()
            .copied()
            .filter(|point| {
                let is_active = self.0.contains(point);
                let active_neighbors = active_neighbors(point);

                match is_active {
                    true => active_neighbors == 2 || active_neighbors == 3,
                    false => active_neighbors == 3,
                }
            })
            .collect();

        ActiveCubes(next_active)
    }

    pub fn boot(&self) -> ActiveCubes {
        (0..6).fold(self.clone(), |prev_state, _| prev_state.cycle())
    }
}

#[cfg(test)]
mod part_one {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(ActiveCubes::parse(TEST_INPUT.as_slice()).count(), 5);
    }

    #[test]
    fn test_one_cycle() {
        assert_eq!(
            ActiveCubes::parse(TEST_INPUT.as_slice()).cycle().count(),
            11
        );
    }

    #[test]
    fn test_case() {
        assert_eq!(
            ActiveCubes::parse(TEST_INPUT.as_slice()).boot().count(),
            112
        );
    }

    #[test]
    fn answer() {
        assert_eq!(
            ActiveCubes::parse(PUZZLE_INPUT.as_slice()).boot().count(),
            247
        );
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
