// Day 01: Report Repair

use shared::prelude::*;
use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;

pub mod imperative;

lazy_static! {
    static ref PUZZLE_INPUT: Vec<i32> =
        parse_input(&puzzle_input::lines(include_str!("puzzle_input.txt"))).unwrap();
}

pub fn parse_input(input: &[&str]) -> Result<Vec<i32>, ParseIntError> {
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

pub fn correct_expense_report_mk_2(entries: &[i32]) -> Option<i32> {
    let mut options = entries.iter().copied().flat_map(|i| {
        entries
            .iter()
            .copied()
            .filter(move |i2| &i != i2)
            .flat_map(move |i2| {
                entries
                    .iter()
                    .copied()
                    .filter(move |i3| &i != i3)
                    .map(move |i3| (i, i2, i3))
            })
    });

    options
        .find(|(i, i2, i3)| i + i2 + i3 == 2020)
        .map(|(i, i2, i3)| i * i2 * i3)
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

#[cfg(test)]
mod part_two {
    use super::*;
    #[test]
    fn test_cases() {
        let test_data = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(correct_expense_report_mk_2(&test_data), Some(241861950));
    }
    #[test]
    fn answer() {
        assert_eq!(correct_expense_report_mk_2(&PUZZLE_INPUT), Some(214486272));
    }
}
