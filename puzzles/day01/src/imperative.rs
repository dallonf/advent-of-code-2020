use shared::prelude::*;
use std::num::ParseIntError;
use std::str::FromStr;

lazy_static! {
    static ref PUZZLE_INPUT: Vec<i32> =
        parse_input(&puzzle_input::lines(include_str!("puzzle_input.txt"))).unwrap();
}

fn parse_input(input: &[&str]) -> Result<Vec<i32>, ParseIntError> {
    input.iter().map(|x| i32::from_str(x)).collect()
}

pub fn correct_expense_report(entries: &[i32]) -> Option<i32> {
    for i in entries.iter().copied() {
        for i2 in entries.iter().copied().filter(move |i2| i2 != &i) {
            if i + i2 == 2020 {
                return Some(i * i2);
            }
        }
    }
    None
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
