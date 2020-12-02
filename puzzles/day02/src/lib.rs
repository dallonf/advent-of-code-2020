// Day 02: Password Philosophy

use regex::Regex;
use shared::prelude::*;
use std::str::FromStr;

lazy_static! {
    static ref PUZZLE_INPUT: Vec<String> = puzzle_input::lines(include_str!("puzzle_input.txt"));
    static ref PUZZLE_INPUT_PARSED: Vec<PasswordEntry> = parse_lines(
        &PUZZLE_INPUT
            .as_slice()
            .iter()
            .map(|x| x.as_str())
            .collect::<Vec<&str>>()
    )
    .unwrap();
    static ref REGEX: Regex = Regex::new(r"(\d+)\-(\d+) ([a-z]): ([a-z]+)").unwrap();
}

#[derive(Debug, Eq, PartialEq)]
pub struct PasswordEntry {
    min: u8,
    max: u8,
    char: char,
    password: String,
}

pub fn parse_lines(lines: &[&str]) -> Result<Vec<PasswordEntry>, String> {
    lines.iter().map(|x| parse_line(x)).collect()
}

pub fn parse_line(line: &str) -> Result<PasswordEntry, String> {
    let result = REGEX
        .captures(line)
        .ok_or(format!("Can't parse line: {}", line))?;

    Ok(PasswordEntry {
        min: u8::from_str(&result[1]).unwrap(),
        max: u8::from_str(&result[2]).unwrap(),
        char: char::from_str(&result[3]).unwrap(),
        password: result[4].to_string(),
    })
}

#[cfg(test)]
mod part_one {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("1-3 a: abcde"),
            Ok(PasswordEntry {
                min: 1,
                max: 3,
                char: 'a',
                password: "abcde".into()
            })
            .into()
        )
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
