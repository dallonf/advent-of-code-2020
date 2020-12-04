// Day 4: Passport Processing

use shared::prelude::*;
use std::collections::HashMap;

type PassportRecord = HashMap<String, String>;

lazy_static! {
    static ref PUZZLE_INPUT: Vec<PassportRecord> = parse_input(&puzzle_input::to_strs(
        &puzzle_input::lines(include_str!("puzzle_input.txt"))
    ));
    static ref TEST_INPUT: Vec<PassportRecord> = parse_input(&puzzle_input::to_strs(
        &puzzle_input::lines(include_str!("test_input.txt"))
    ));
    static ref REQUIRED_ENTRIES: Vec<&'static str> =
        vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
}

pub fn parse_input(input_lines: &[&str]) -> Vec<PassportRecord> {
    input_lines
        .split(|x| x.trim().is_empty())
        .map(|definition_lines| {
            definition_lines
                .join(" ")
                .split(" ")
                .map(|field| {
                    let pair: Vec<_> = field.split(":").collect();
                    (pair[0].to_string(), pair[1].to_string())
                })
                .collect()
        })
        .collect()
}

pub fn is_valid(passport: &PassportRecord) -> bool {
    REQUIRED_ENTRIES
        .iter()
        .all(|x| passport.contains_key(&x.to_string()))
}

pub fn valid_passports(passports: &[PassportRecord]) -> usize {
    passports.iter().filter(|x| is_valid(x)).count()
}

#[cfg(test)]
mod part_one {
    use super::*;

    #[test]
    fn test_cases() {
        let result: Vec<bool> = TEST_INPUT.iter().map(|x| is_valid(x)).collect();
        assert_eq!(result, vec![true, false, true, false]);
        assert_eq!(valid_passports(&TEST_INPUT), 2);
    }

    #[test]
    fn answer() {
        assert_eq!(valid_passports(&PUZZLE_INPUT), 202);
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
