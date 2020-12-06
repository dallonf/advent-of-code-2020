// Day 4: Passport Processing

use regex::Regex;
use shared::prelude::*;
use std::collections::HashMap;

type PassportRecord = HashMap<String, String>;

lazy_static! {
    static ref PUZZLE_INPUT: Vec<PassportRecord> =
        parse_input(&(&puzzle_input::lines(include_str!("puzzle_input.txt"))));
    static ref PART_1_TEST_INPUT: Vec<PassportRecord> =
        parse_input(&&puzzle_input::lines(include_str!("part_1_test_input.txt")));
    static ref PART_2_INVALID_TEST_INPUT: Vec<PassportRecord> =
        parse_input(&&puzzle_input::lines(include_str!("part_2_invalid.txt")));
    static ref PART_2_VALID_TEST_INPUT: Vec<PassportRecord> =
        parse_input(&&puzzle_input::lines(include_str!("part_2_valid.txt")));
    static ref REQUIRED_ENTRIES: HashMap<&'static str, fn(&str) -> bool> = {
        let mut x = HashMap::<&str, fn(&str) -> bool>::new();
        x.insert("byr", is_valid_byr);
        x.insert("iyr", is_valid_iyr);
        x.insert("eyr", is_valid_eyr);
        x.insert("hgt", is_valid_hgt);
        x.insert("hcl", is_valid_hcl);
        x.insert("ecl", is_valid_ecl);
        x.insert("pid", is_valid_pid);
        x
    };
    static ref HGT_REGEX: Regex = Regex::new(r"^([0-9]+)(cm|in)$").unwrap();
    static ref HCL_REGEX: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    static ref PID_REGEX: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
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
        .keys()
        .all(|x| passport.contains_key(&x.to_string()))
}

pub fn valid_passports(passports: &[PassportRecord]) -> usize {
    passports.iter().filter(|x| is_valid(x)).count()
}

pub fn is_valid_byr(input: &str) -> bool {
    let parsed: u16 = input.parse().unwrap();
    parsed >= 1920 && parsed <= 2002
}
pub fn is_valid_iyr(input: &str) -> bool {
    let parsed: u16 = input.parse().unwrap();
    parsed >= 2010 && parsed <= 2020
}
pub fn is_valid_eyr(input: &str) -> bool {
    let parsed: u16 = input.parse().unwrap();
    parsed >= 2020 && parsed <= 2030
}
pub fn is_valid_hgt(input: &str) -> bool {
    let regex_match = HGT_REGEX.captures(input);
    regex_match.map_or(false, |regex_match| {
        let parsed: u16 = regex_match[1].parse().unwrap();
        match &regex_match[2] {
            "cm" => parsed >= 150 && parsed <= 193,
            "in" => parsed >= 59 && parsed <= 76,
            _ => false,
        }
    })
}
pub fn is_valid_hcl(input: &str) -> bool {
    HCL_REGEX.is_match(input)
}
pub fn is_valid_ecl(input: &str) -> bool {
    match input {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
        _ => false,
    }
}
pub fn is_valid_pid(input: &str) -> bool {
    PID_REGEX.is_match(input)
}

pub fn is_valid_mk_2(passport: &PassportRecord) -> bool {
    REQUIRED_ENTRIES.iter().all(|(key, validator)| {
        passport
            .get(&key.to_string())
            .map_or(false, |x| validator(x))
    })
}

#[cfg(test)]
mod part_one {
    use super::*;

    #[test]
    fn test_cases() {
        let result: Vec<bool> = PART_1_TEST_INPUT.iter().map(|x| is_valid(x)).collect();
        assert_eq!(result, vec![true, false, true, false]);
        assert_eq!(valid_passports(&PART_1_TEST_INPUT), 2);
    }

    #[test]
    fn answer() {
        assert_eq!(valid_passports(&PUZZLE_INPUT), 202);
    }
}

#[cfg(test)]
mod part_two {
    use super::*;
    #[test]
    fn test_individual_validators() {
        assert_eq!(is_valid_byr("2002"), true);
        assert_eq!(is_valid_byr("2003"), false);

        assert_eq!(is_valid_hgt("60in"), true);
        assert_eq!(is_valid_hgt("190cm"), true);
        assert_eq!(is_valid_hgt("190in"), false);
        assert_eq!(is_valid_hgt("190"), false);

        assert_eq!(is_valid_hcl("#123abc"), true);
        assert_eq!(is_valid_hcl("#123abz"), false);
        assert_eq!(is_valid_hcl("123abc"), false);

        assert_eq!(is_valid_ecl("brn"), true);
        assert_eq!(is_valid_ecl("wat"), false);

        assert_eq!(is_valid_pid("000000001"), true);
        assert_eq!(is_valid_pid("0123456789"), false);
    }

    #[test]
    fn test_invalid_examples() {
        assert!(PART_2_INVALID_TEST_INPUT.iter().all(|x| !is_valid_mk_2(x)))
    }

    #[test]
    fn test_valid_examples() {
        assert!(PART_2_VALID_TEST_INPUT.iter().all(is_valid_mk_2))
    }

    #[test]
    fn answer() {
        assert_eq!(
            PUZZLE_INPUT.iter().filter(|x| is_valid_mk_2(x)).count(),
            137
        )
    }
}
