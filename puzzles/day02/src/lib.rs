// Day 02: Password Philosophy

use regex::Regex;
use shared::prelude::*;
use std::error::Error;
use std::str::FromStr;

lazy_static! {
    static ref PUZZLE_INPUT: Vec<&'static str> =
        puzzle_input::lines(include_str!("puzzle_input.txt"));
    static ref PUZZLE_INPUT_PARSED: Vec<PasswordEntry> = parse_lines(&PUZZLE_INPUT).unwrap();
    static ref REGEX: Regex = Regex::new(r"(\d+)\-(\d+) ([a-z]): ([a-z]+)").unwrap();
}

#[derive(Debug, Eq, PartialEq)]
pub struct PasswordEntry {
    min: usize,
    max: usize,
    validate_char: char,
    password: String,
}

pub fn parse_lines(lines: &[&str]) -> Result<Vec<PasswordEntry>, Box<dyn Error>> {
    lines.iter().map(|x| parse_line(x)).collect()
}

pub fn parse_line(line: &str) -> Result<PasswordEntry, Box<dyn Error>> {
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

pub fn password_is_valid_mk_2(input: &PasswordEntry) -> bool {
    let char_at_min = input.password.chars().nth(input.min - 1).unwrap() == input.validate_char;
    let char_at_max = input.password.chars().nth(input.max - 1).unwrap() == input.validate_char;
    (char_at_min || char_at_max) && !(char_at_min && char_at_max)
}

pub fn count_valid_passwords_mk_2(
    input: &[PasswordEntry],
    validator: fn(&PasswordEntry) -> bool,
) -> usize {
    input.iter().filter(|x| validator(x)).count()
}

#[cfg(test)]
mod part_one {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("1-3 a: abcde").unwrap(),
            PasswordEntry {
                min: 1,
                max: 3,
                validate_char: 'a',
                password: "abcde".into()
            }
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

#[cfg(test)]
mod part_two {
    use super::*;

    #[test]
    fn test_password_is_valid() {
        assert_eq!(
            password_is_valid_mk_2(&parse_line("1-3 a: abcde").unwrap()),
            true,
            "1-3 a: abcde"
        );
        assert_eq!(
            password_is_valid_mk_2(&parse_line("2-9 c: ccccccccc").unwrap()),
            false,
            "2-9 c: ccccccccc"
        );
    }

    #[test]
    fn test_case() {
        let input = parse_lines(&vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"]).unwrap();
        assert_eq!(
            count_valid_passwords_mk_2(&input, password_is_valid_mk_2),
            1
        );
    }

    #[test]
    fn answer() {
        assert_eq!(
            count_valid_passwords_mk_2(PUZZLE_INPUT_PARSED.as_ref(), password_is_valid_mk_2),
            294
        );
    }
}
