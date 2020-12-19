// Day 19: Monster Messages

use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
    str::FromStr,
};

use shared::prelude::*;

#[derive(Debug)]
pub struct Input<'a>(Rules, Vec<&'a str>);

#[derive(Debug, Clone)]
pub struct Rules {
    rule_map: HashMap<usize, Rule>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rule {
    LiteralChar(char),
    RuleLists(Vec<Vec<usize>>),
}

lazy_static! {
    static ref PUZZLE_INPUT: Input<'static> =
        Input::parse(&puzzle_input::lines(include_str!("puzzle_input.txt"))).unwrap();
    static ref TEST_INPUT: Input<'static> =
        Input::parse(&puzzle_input::lines(include_str!("test_input.txt"))).unwrap();
    static ref TEST_INPUT_2: Input<'static> =
        Input::parse(&puzzle_input::lines(include_str!("test_input_2.txt"))).unwrap();
}

impl Input<'_> {
    pub fn parse<'a>(input: &[&'a str]) -> anyhow::Result<Input<'a>> {
        let sections: Vec<_> = input.split(|x| x.trim().is_empty()).collect();
        if sections.len() != 2 {
            return Err(anyhow!("wrong number of sections"));
        }

        let rules = Rules::parse(sections[0])?;
        let values = sections[1];

        Ok(Input(rules, values.to_vec()))
    }
}

lazy_static! {
    static ref RULE_REGEX: Regex = Regex::new(r"^([0-9]+): (.+)$").unwrap();
    static ref LITERAL_CHAR_REGEX: Regex = Regex::new(r#"^"([a-z])"$"#).unwrap();
}

#[derive(Debug)]
enum RuleParseResult<'a> {
    Matches { possible_leftovers: Vec<&'a str> },
    DoesNotMatch,
}

impl<'a> FromIterator<RuleParseResult<'a>> for RuleParseResult<'a> {
    fn from_iter<T: IntoIterator<Item = RuleParseResult<'a>>>(iter: T) -> Self {
        iter.into_iter()
            .fold(RuleParseResult::DoesNotMatch, |prev, next| {
                match (prev, next) {
                    (
                        RuleParseResult::Matches {
                            mut possible_leftovers,
                        },
                        RuleParseResult::Matches {
                            possible_leftovers: mut next_leftovers,
                        },
                    ) => {
                        possible_leftovers.append(&mut next_leftovers);
                        RuleParseResult::Matches { possible_leftovers }
                    }
                    (result @ RuleParseResult::Matches { .. }, RuleParseResult::DoesNotMatch) => {
                        result
                    }
                    (RuleParseResult::DoesNotMatch, result @ RuleParseResult::Matches { .. }) => {
                        result
                    }
                    (result @ RuleParseResult::DoesNotMatch, RuleParseResult::DoesNotMatch) => {
                        result
                    }
                }
            })
    }
}

impl Rules {
    pub fn parse(rules: &[&str]) -> anyhow::Result<Rules> {
        let rule_map: anyhow::Result<HashMap<usize, Rule>> = rules
            .iter()
            .map(|x| {
                let captures = RULE_REGEX
                    .captures(x)
                    .ok_or(anyhow!("bad rule formatting"))?;

                let rule_id: usize = captures[1].parse()?;
                let rule: Rule = captures[2].parse()?;

                Ok((rule_id, rule))
            })
            .collect();

        Ok(Rules {
            rule_map: rule_map?,
        })
    }

    pub fn mk2_patch(mut self) -> Rules {
        self.rule_map.insert(8, "42 | 42 8".parse().unwrap());
        self.rule_map
            .insert(11, "42 31 | 42 11 31".parse().unwrap());

        self
    }

    pub fn matches(&self, s: &str) -> bool {
        match self.matches_rule(s, self.rule_map.get(&0).unwrap()) {
            RuleParseResult::Matches { possible_leftovers } => {
                possible_leftovers.iter().any(|x| x.is_empty())
            }
            RuleParseResult::DoesNotMatch => false,
        }
    }

    fn matches_rule_list<'a>(&self, s: &'a str, list: &[usize]) -> RuleParseResult<'a> {
        if list.is_empty() {
            return RuleParseResult::Matches {
                possible_leftovers: vec![s],
            };
        }
        let (next_rule_list, other_rules) = list.split_first().unwrap();
        let next_rule = if let Some(x) = self.rule_map.get(next_rule_list) {
            x
        } else {
            return RuleParseResult::DoesNotMatch;
        };

        if let RuleParseResult::Matches { possible_leftovers } = self.matches_rule(s, next_rule) {
            possible_leftovers
                .iter()
                .map(|leftover| self.matches_rule_list(leftover, other_rules))
                .collect()
        } else {
            RuleParseResult::DoesNotMatch
        }
    }

    fn matches_rule<'a>(&self, s: &'a str, rule: &Rule) -> RuleParseResult<'a> {
        match rule {
            Rule::LiteralChar(char) => {
                if let Some(leftover) = s.strip_prefix(*char) {
                    RuleParseResult::Matches {
                        possible_leftovers: vec![leftover],
                    }
                } else {
                    RuleParseResult::DoesNotMatch
                }
            }
            Rule::RuleLists(alternatives) => alternatives
                .iter()
                .map(|rule_list| self.matches_rule_list(s, rule_list))
                .collect(),
        }
    }

    #[allow(dead_code)]
    fn debug_rule(&self, rule_id: usize, explored_rules: &mut HashSet<usize>) -> String {
        if explored_rules.contains(&rule_id) {
            return format!("[{}]", rule_id);
        }
        explored_rules.insert(rule_id);

        let rule = if let Some(x) = self.rule_map.get(&rule_id) {
            x
        } else {
            return format!("[{}]", rule_id);
        };

        match rule {
            Rule::LiteralChar(char) => char.to_string(),
            Rule::RuleLists(lists) => {
                let options = lists
                    .iter()
                    .map(|x| {
                        x.iter()
                            .map(|y| self.debug_rule(*y, &mut explored_rules.clone()))
                            .collect::<Vec<String>>()
                            .join("")
                    })
                    .collect::<Vec<String>>();
                if options.len() == 1 {
                    options[0].to_owned()
                } else {
                    format!("({})", options.join("|"))
                }
            }
        }
    }
}

impl FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(literal_char_captures) = LITERAL_CHAR_REGEX.captures(s) {
            Ok(Rule::LiteralChar(
                literal_char_captures[1].chars().nth(0).unwrap(),
            ))
        } else {
            let rule_lists: Result<Vec<Vec<usize>>, _> = s
                .split(" | ")
                .map(|alternative| {
                    alternative
                        .split(" ")
                        .map(|x| x.parse().map_err(|e| anyhow::Error::from(e)))
                        .collect()
                })
                .collect();

            Ok(Rule::RuleLists(rule_lists?))
        }
    }
}

#[cfg(test)]
mod part_one {
    use super::*;

    #[test]
    fn test_cases() {
        let Input(rules, values) = TEST_INPUT.deref();
        let matches: Vec<&str> = values
            .iter()
            .copied()
            .filter(|x| rules.matches(x))
            .collect();
        assert_eq!(matches, vec!["ababbb", "abbbab"]);
    }

    #[test]
    fn answer() {
        let Input(rules, values) = PUZZLE_INPUT.deref();
        let matches = values.iter().copied().filter(|x| rules.matches(x)).count();
        assert_eq!(matches, 144);
    }
}

#[cfg(test)]
mod part_two {
    use super::*;

    #[test]
    fn confirm_unpatched_behavior() {
        let Input(rules, values) = TEST_INPUT_2.deref();
        let matches: Vec<&str> = values
            .iter()
            .copied()
            .filter(|x| rules.matches(x))
            .collect();
        assert_eq!(
            matches,
            vec!["bbabbbbaabaabba", "ababaaaaaabaaab", "ababaaaaabbbaba"]
        );
    }

    #[test]
    fn specific_test() {
        let Input(rules, _) = TEST_INPUT_2.deref();
        let rules = rules.to_owned().mk2_patch();
        assert!(rules.matches("babbbbaabbbbbabbbbbbaabaaabaaa"));
    }

    #[test]
    fn test_case() {
        let Input(rules, values) = TEST_INPUT_2.deref();
        let rules = rules.to_owned().mk2_patch();
        let matches: Vec<&str> = values
            .iter()
            .copied()
            .filter(|x| rules.matches(x))
            .collect();
        assert_eq!(
            matches,
            vec![
                "bbabbbbaabaabba",
                "babbbbaabbbbbabbbbbbaabaaabaaa",
                "aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
                "bbbbbbbaaaabbbbaaabbabaaa",
                "bbbababbbbaaaaaaaabbababaaababaabab",
                "ababaaaaaabaaab",
                "ababaaaaabbbaba",
                "baabbaaaabbaaaababbaababb",
                "abbbbabbbbaaaababbbbbbaaaababb",
                "aaaaabbaabaaaaababaa",
                "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa",
                "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba",
            ]
        );
    }

    #[test]
    fn answer() {
        let Input(rules, values) = PUZZLE_INPUT.deref();
        let rules = rules.to_owned().mk2_patch();
        let matches = values.iter().copied().filter(|x| rules.matches(x)).count();
        assert_eq!(matches, 260);
    }
}
