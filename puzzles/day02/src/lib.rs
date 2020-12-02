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
    min: usize,
    max: usize,
    validate_char: char,
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
        min: usize::from_str(&result[1]).unwrap(),
        max: usize::from_str(&result[2]).unwrap(),
        validate_char: char::from_str(&result[3]).unwrap(),
        password: result[4].to_string(),
    })
}

pub fn count_valid_passwords(input: &[PasswordEntry]) -> usize {
    input.iter().filter(|x| password_is_valid(x)).count()
}

pub fn password_is_valid(input: &PasswordEntry) -> bool {
    let char_count = input
        .password
        .chars()
        .filter(|x| x == &input.validate_char)
        .count();
    char_count >= input.min && char_count <= input.max
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
                validate_char: 'a',
                password: "abcde".into()
            })
            .into()
        )
    }

    #[test]
    fn test_password_is_valid() {
        assert_eq!(
            password_is_valid(&parse_line("1-3 a: abcde").unwrap()),
            true
        );
    }

    #[test]
    fn test_case() {
        let input = parse_lines(&vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"]).unwrap();
        assert_eq!(count_valid_passwords(&input), 2);
    }

    #[test]
    fn answer() {
        assert_eq!(count_valid_passwords(PUZZLE_INPUT_PARSED.as_ref()), 465);
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
