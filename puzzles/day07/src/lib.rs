// Day 7: Handy Haversacks

use anyhow::anyhow;
use regex::Regex;
use shared::prelude::*;
use std::convert::From;
use std::str::FromStr;
use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
};

#[derive(Eq, PartialEq, Debug, Hash, Clone)]
pub struct BagCollection(i32, String);

#[derive(Eq, PartialEq, Debug)]
pub struct BagRule {
    color: String,
    contains: HashSet<BagCollection>,
}

pub struct BagRuleGraph {
    children: HashMap<String, HashSet<BagCollection>>,
    parents: HashMap<String, HashMap<String, i32>>,
}

lazy_static! {
    static ref TEST_INPUT: Vec<BagRule> = puzzle_input::lines(include_str!("test_input.txt"))
        .into_iter()
        .map(BagRule::from_str)
        .collect::<anyhow::Result<Vec<BagRule>>>()
        .unwrap();
    static ref PUZZLE_INPUT: Vec<BagRule> = puzzle_input::lines(include_str!("puzzle_input.txt"))
        .into_iter()
        .map(BagRule::from_str)
        .collect::<anyhow::Result<Vec<BagRule>>>()
        .unwrap();
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
            contains: allowed?,
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

        Ok(BagCollection(matches[1].parse()?, matches[2].into()))
    }
}

impl From<&[BagRule]> for BagRuleGraph {
    fn from(input: &[BagRule]) -> Self {
        let children = input
            .iter()
            .map(|rule| (rule.color.clone(), rule.contains.iter().cloned().collect()))
            .collect();

        let mut parents = HashMap::new();
        for rule in input.iter() {
            for BagCollection(n, child_color) in rule.contains.iter() {
                if !parents.contains_key(child_color) {
                    parents.insert(child_color.clone(), HashMap::new());
                }
                let child_entry = parents.get_mut(child_color).unwrap();

                child_entry.insert(rule.color.clone(), *n);
            }
        }

        BagRuleGraph { children, parents }
    }
}

pub fn get_possible_outer_bags(
    inner_bag_color: &str,
    bag_rules: &BagRuleGraph,
    cache: &mut HashMap<String, HashSet<String>>,
) -> HashSet<String> {
    match cache.get(inner_bag_color) {
        Some(result) => result.clone(),
        None => {
            let result: HashSet<String> = {
                let possible_parents = match bag_rules.parents.get(inner_bag_color) {
                    Some(x) => Cow::Borrowed(x),
                    None => Cow::Owned(HashMap::new()),
                };

                possible_parents
                    .iter()
                    .flat_map(|(parent_color, _)| -> Vec<String> {
                        get_possible_outer_bags(parent_color, bag_rules, cache)
                            .into_iter()
                            .chain(std::iter::once(parent_color.clone()))
                            .collect()
                    })
                    .collect::<HashSet<String>>()
            };
            cache.insert(inner_bag_color.to_string(), result.clone());
            result
        }
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
                contains: vec![
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
                contains: HashSet::new(),
            }
        );
    }

    #[test]
    fn test_cases() {
        let result = get_possible_outer_bags(
            "shiny gold",
            &BagRuleGraph::from(TEST_INPUT.as_ref()),
            &mut HashMap::new(),
        );

        assert_eq!(result.len(), 4);
    }

    #[test]
    fn answer() {
        let result = get_possible_outer_bags(
            "shiny gold",
            &BagRuleGraph::from(PUZZLE_INPUT.as_ref()),
            &mut HashMap::new(),
        );

        assert_eq!(result.len(), 155);
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
