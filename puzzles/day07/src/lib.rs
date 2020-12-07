// Day 7: Handy Haversacks

use anyhow::anyhow;
use regex::Regex;
use shared::prelude::*;
use std::collections::{HashMap, HashSet};
use std::convert::From;
use std::hash::Hash;
use std::str::FromStr;

#[derive(Eq, PartialEq, Debug, Hash, Clone)]
pub struct BagCollection(usize, String);

#[derive(Eq, PartialEq, Debug)]
pub struct BagRule {
    color: String,
    contains: HashSet<BagCollection>,
}

pub struct BagRuleGraph {
    children: HashMap<String, HashSet<BagCollection>>,
    parents: HashMap<String, HashMap<String, usize>>,
}

lazy_static! {
    static ref TEST_INPUT: Vec<BagRule> = puzzle_input::lines(include_str!("test_input.txt"))
        .into_iter()
        .map(BagRule::from_str)
        .collect::<anyhow::Result<Vec<BagRule>>>()
        .unwrap();
    static ref TEST_INPUT_2: Vec<BagRule> = puzzle_input::lines(include_str!("test_input_2.txt"))
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

pub fn get_possible_outer_bags<'a>(
    inner_bag_color: &'_ str,
    bag_rules: &'a BagRuleGraph,
    cache: &mut HashMap<String, HashSet<&'a str>>,
) -> HashSet<&'a str> {
    match cache.get(inner_bag_color) {
        Some(result) => result.to_owned(),
        None => {
            let result: HashSet<&str> = match bag_rules.parents.get(inner_bag_color) {
                Some(possible_parents) => possible_parents
                    .iter()
                    .flat_map(|(parent_color, _)| -> Vec<&str> {
                        get_possible_outer_bags(parent_color, bag_rules, cache)
                            .into_iter()
                            .chain(std::iter::once(parent_color.as_str()))
                            .collect()
                    })
                    .collect::<HashSet<&str>>(),
                None => HashSet::new(),
            };
            cache.insert(inner_bag_color.to_owned(), result.to_owned());
            result
        }
    }
}

pub fn get_total_contained_bags(
    outer_bag_color: &str,
    bag_rules: &BagRuleGraph,
    cache: &mut HashMap<String, usize>,
) -> usize {
    match cache.get(outer_bag_color) {
        Some(result) => *result,
        None => {
            let result = match bag_rules.children.get(outer_bag_color) {
                Some(bag_children) => bag_children
                    .iter()
                    .map(|BagCollection(n, inner_color)| {
                        n * (1_usize + get_total_contained_bags(inner_color, &bag_rules, cache))
                    })
                    .sum(),
                None => 0_usize,
            };

            cache.insert(outer_bag_color.to_owned(), result);
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
        let rules = BagRuleGraph::from(TEST_INPUT.as_ref());
        let result = get_possible_outer_bags("shiny gold", &rules, &mut HashMap::new());

        assert_eq!(result.len(), 4);
    }

    #[test]
    fn answer() {
        let rules = BagRuleGraph::from(PUZZLE_INPUT.as_ref());
        let result = get_possible_outer_bags("shiny gold", &rules, &mut HashMap::new());

        assert_eq!(result.len(), 155);
    }
}

#[cfg(test)]
mod part_two {
    use super::*;
    #[test]
    fn test_case() {
        let result = get_total_contained_bags(
            "shiny gold",
            &BagRuleGraph::from(TEST_INPUT.as_ref()),
            &mut HashMap::new(),
        );
        assert_eq!(result, 32);
    }

    #[test]
    fn test_case_2() {
        let result = get_total_contained_bags(
            "shiny gold",
            &BagRuleGraph::from(TEST_INPUT_2.as_ref()),
            &mut HashMap::new(),
        );
        assert_eq!(result, 126);
    }

    #[test]
    fn answer() {
        let result = get_total_contained_bags(
            "shiny gold",
            &BagRuleGraph::from(PUZZLE_INPUT.as_ref()),
            &mut HashMap::new(),
        );
        assert_eq!(result, 54803);
    }
}
