// Day 3: Toboggan Trajectory

use shared::prelude::*;

lazy_static! {
    static ref PUZZLE_INPUT: TreeMap = parse_input(&puzzle_input::to_strs(&puzzle_input::lines(
        include_str!("puzzle_input.txt")
    )));
    static ref TEST_INPUT: TreeMap = parse_input(&puzzle_input::to_strs(&puzzle_input::lines(
        include_str!("test_input.txt")
    )));
}

pub struct TreeMap {
    rows: Vec<Vec<bool>>,
    row_len: usize,
}

impl TreeMap {
    pub fn is_tree(&self, x: usize, y: usize) -> bool {
        let wrapped_x = x % self.row_len;
        self.rows[y][wrapped_x]
    }

    pub fn collisions_along_slope(&self, right_for_each_down: usize) -> usize {
        (0..self.rows.len())
            .filter(|y| {
                let x = y * right_for_each_down;
                self.is_tree(x, *y)
            })
            .count()
    }
}

pub fn parse_input(input: &[&str]) -> TreeMap {
    let rows: Vec<_> = input
        .iter()
        .map(|y| y.chars().map(|x| x == '#').collect::<Vec<_>>())
        .collect();
    let row_len = rows[0].len();
    TreeMap { rows, row_len }
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
        assert_eq!(TEST_INPUT.collisions_along_slope(3), 7);
    }

    #[test]
    fn answer() {
        assert_eq!(PUZZLE_INPUT.collisions_along_slope(3), 230);
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
