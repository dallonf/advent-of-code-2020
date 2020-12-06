// Day 3: Toboggan Trajectory

use std::vec;

use shared::prelude::*;

pub struct TreeMap {
    rows: Vec<Vec<bool>>,
    row_len: usize,
}

pub type Slope = (usize, usize);

lazy_static! {
    static ref PUZZLE_INPUT: TreeMap =
        parse_input(&puzzle_input::lines(include_str!("puzzle_input.txt")));
    static ref TEST_INPUT: TreeMap =
        parse_input(&puzzle_input::lines(include_str!("test_input.txt")));
    static ref SLOPES: Vec<Slope> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
}

pub fn parse_input(input: &[&str]) -> TreeMap {
    let rows: Vec<_> = input
        .iter()
        .map(|y| y.chars().map(|x| x == '#').collect::<Vec<_>>())
        .collect();
    let row_len = rows[0].len();
    TreeMap { rows, row_len }
}

impl TreeMap {
    pub fn is_tree(&self, x: usize, y: usize) -> bool {
        let wrapped_x = x % self.row_len;
        self.rows[y][wrapped_x]
    }

    pub fn collisions_along_slope(&self, (right, down): Slope) -> usize {
        (0..self.rows.len())
            .step_by(down)
            .enumerate()
            .filter(|(i, y)| {
                let x = i * right;
                self.is_tree(x, *y)
            })
            .count()
    }
}

pub fn collisions_multiplied_along_slopes(tree_map: &TreeMap, slopes: &[Slope]) -> usize {
    let collisions = slopes
        .iter()
        .copied()
        .map(|x| tree_map.collisions_along_slope(x));

    collisions
        .fold(None, |prev, x| match prev {
            Some(y) => Some(y * x),
            None => Some(x),
        })
        .unwrap_or(0)
}

#[cfg(test)]
mod part_one {
    use super::*;

    #[test]
    fn test_is_tree() {
        assert!(TEST_INPUT.is_tree(3, 1) == false);
        assert!(TEST_INPUT.is_tree(6, 2));
    }

    #[test]
    fn test_collisions() {
        assert_eq!(TEST_INPUT.collisions_along_slope((3, 1)), 7);
    }

    #[test]
    fn answer() {
        assert_eq!(PUZZLE_INPUT.collisions_along_slope((3, 1)), 230);
    }
}

#[cfg(test)]
mod part_two {
    use super::*;
    #[test]
    fn test_down_2() {
        assert_eq!(TEST_INPUT.collisions_along_slope((1, 2)), 2);
    }

    #[test]
    fn test_case() {
        assert_eq!(
            collisions_multiplied_along_slopes(&TEST_INPUT, &SLOPES),
            336
        );
    }

    #[test]
    fn answer() {
        assert_eq!(
            collisions_multiplied_along_slopes(&PUZZLE_INPUT, &SLOPES),
            9533698720
        );
    }
}
