// Day 6: Custom Customs

use std::collections::{HashMap, HashSet};

use shared::prelude::*;

lazy_static! {
    static ref PUZZLE_INPUT: Vec<Vec<&'static str>> =
        parse_groups(&puzzle_input::lines_strs(include_str!("puzzle_input.txt")));
    static ref TEST_INPUT: Vec<Vec<&'static str>> =
        parse_groups(&puzzle_input::lines_strs(include_str!("test_input.txt")));
}

pub fn parse_groups<'a>(inputs: &[&'a str]) -> Vec<Vec<&'a str>> {
    inputs
        .split(|x| x.trim().is_empty())
        .map(|x| x.to_vec())
        .collect()
}

pub fn unique_answers_per_group<'a, TGroup: AsRef<[&'a str]>>(groups: &[TGroup]) -> usize {
    groups.iter().map(|x| unique_answers(x.as_ref())).sum()
}

fn unique_answers(group: &[&str]) -> usize {
    let group: Vec<&str> = group.iter().map(|x| x.as_ref()).collect();
    group
        .join("")
        .chars()
        .fold(HashSet::new(), |mut set, char| {
            set.insert(char);
            set
        })
        .len()
}

pub fn unanimous_answers_per_group<'a, TGroup: AsRef<[&'a str]>>(groups: &[TGroup]) -> usize {
    groups.iter().map(|x| unanimous_answers(&x.as_ref())).sum()
}

fn unanimous_answers(group: &[&str]) -> usize {
    let group: Vec<&str> = group.iter().map(|x| x.as_ref()).collect();
    group
        .join("")
        .chars()
        .fold(HashMap::new(), |mut set, char| {
            set.insert(char, set.get(&char).map_or(1, |x| x + 1));
            set
        })
        .into_iter()
        .filter(|(_, answers)| *answers == group.len())
        .count()
}

#[cfg(test)]
mod part_one {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(TEST_INPUT.len(), 5);
    }

    #[test]
    fn test_case() {
        assert_eq!(unique_answers_per_group(TEST_INPUT.as_ref()), 11)
    }

    #[test]
    fn answer() {
        assert_eq!(unique_answers_per_group(PUZZLE_INPUT.as_ref()), 6748)
    }
}

#[cfg(test)]
mod part_two {
    use super::*;
    #[test]
    fn test_case() {
        assert_eq!(unanimous_answers_per_group(TEST_INPUT.as_ref()), 6)
    }
    #[test]
    fn answer() {
        assert_eq!(unanimous_answers_per_group(PUZZLE_INPUT.as_ref()), 3445)
    }
}
