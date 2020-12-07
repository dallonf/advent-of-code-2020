// Day 7: Handy Haversacks

use anyhow::anyhow;
use regex::Regex;
use shared::prelude::*;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Eq, PartialEq, Debug, Hash)]
struct BagCollection(i32, String);

#[derive(Eq, PartialEq, Debug)]
struct BagRule {
    color: String,
    allowed: HashSet<BagCollection>,
}

lazy_static! {
    static ref TEST_INPUT: Vec<&'static str> =
        puzzle_input::lines(include_str!("test_input.txt"));

    // static ref PUZZLE_INPUT: Vec<&'static str> =
    //     puzzle_input::lines(include_str!("puzzle_input.txt"));
}

lazy_static! {
    static ref BAG_REGEX: Regex = Regex::new(r"^([a-z ]+?) bags contain ([0-9 a-z,]+)\.").unwrap();
    static ref BAG_COLLECTION_REGEX: Regex = Regex::new(r"^([0-9]+) ([a-z ]+?) bags?$").unwrap();
}
impl FromStr for BagRule {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let matches = match BAG_REGEX.captures(input) {
            Some(matches) => Ok(matches),
            None => Err(anyhow!("Didn't match regex: {}", input)),
        }?;

        let allowed: Result<HashSet<BagCollection>, anyhow::Error> = match &matches[2] {
            "no other bags" => Ok(HashSet::new()),
            list => list.split(", ").map(BagCollection::from_str).collect(),
        };

        Ok(BagRule {
            color: matches[1].to_string(),
            allowed: allowed?,
        })
    }
}

impl FromStr for BagCollection {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let matches = match BAG_COLLECTION_REGEX.captures(input) {
            Some(matches) => Ok(matches),
            None => Err(anyhow!("Didn't match regex: {}", input)),
        }?;

        println!("{:?}", matches);

        Ok(BagCollection(matches[1].parse()?, matches[2].into()))
    }
}

#[cfg(test)]
mod part_one {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            BagRule::from_str("light red bags contain 1 bright white bag, 2 muted yellow bags.")
                .unwrap(),
            BagRule {
                color: "light red".to_string(),
                allowed: vec![
                    BagCollection(1, "bright white".to_string()),
                    BagCollection(2, "muted yellow".to_string())
                ]
                .into_iter()
                .collect()
            }
        );

        assert_eq!(
            BagRule::from_str("faded blue bags contain no other bags.").unwrap(),
            BagRule {
                color: "faded blue".to_string(),
                allowed: HashSet::new(),
            }
        );
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
