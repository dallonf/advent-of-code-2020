// Day 01: Report Repair

use shared::prelude::*;
use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;

lazy_static! {
    static ref PUZZLE_INPUT: Vec<i32> =
        parse_input(&puzzle_input::lines(include_str!("puzzle_input.txt"))).unwrap();
}

pub fn parse_input(input: &[String]) -> Result<Vec<i32>, ParseIntError> {
    input.iter().map(|x| i32::from_str(x)).collect()
}

pub fn correct_expense_report(entries: &[i32]) -> Option<i32> {
    let set: HashSet<_> = entries.iter().cloned().collect();
    entries
        .iter()
        .copied()
        .map(|i| (i, 2020 - i))
        .find(|(_, missing)| set.contains(missing))
        .map(|(i, missing)| i * missing)
}

#[cfg(test)]
mod part_one {
    use super::*;

    #[test]
    fn test_cases() {
        let test_data = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(correct_expense_report(&test_data), Some(514579));
    }

    #[test]
    fn answer() {
        assert_eq!(correct_expense_report(&PUZZLE_INPUT), Some(651651));
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
