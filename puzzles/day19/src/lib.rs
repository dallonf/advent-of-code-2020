// Day 19: Monster Messages

use std::{
    collections::{HashMap, HashSet},
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
    static ref INTERESTING_RULES: HashSet<usize> = vec![8, 11, 42, 31].into_iter().collect();
}

#[derive(Debug)]
enum RuleParseResult<'a> {
    Matches { leftover: &'a str },
    DoesNotMatch,
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
            RuleParseResult::Matches { leftover } => leftover.is_empty(),
            RuleParseResult::DoesNotMatch => false,
        }
    }

    fn matches_rule_list<'a>(&self, s: &'a str, list: &[usize]) -> RuleParseResult<'a> {
        if list.is_empty() {
            return RuleParseResult::Matches { leftover: s };
        }
        let (next_rule, other_rules) = list.split_first().unwrap();
        let next_rule = if let Some(x) = self.rule_map.get(next_rule) {
            if INTERESTING_RULES.contains(next_rule) {
                println!("rule ID {} = {:?}", next_rule, x);
            }
            x
        } else {
            return RuleParseResult::DoesNotMatch;
        };

        if let RuleParseResult::Matches { leftover } = self.matches_rule(s, next_rule) {
            self.matches_rule_list(leftover, other_rules)
        } else {
            RuleParseResult::DoesNotMatch
        }
    }

    fn matches_rule<'a>(&self, s: &'a str, rule: &Rule) -> RuleParseResult<'a> {
        let is_interesting = INTERESTING_RULES
            .iter()
            .any(|i| self.rule_map.get(i).map(|x| x == rule).unwrap_or(false));
        if is_interesting {
            let rule_id = INTERESTING_RULES
                .iter()
                .find(|i| self.rule_map.get(i).map(|x| x == rule).unwrap_or(false))
                .unwrap();

            println!("testing if s:{} matches rule: {}", s, rule_id);
            let result = self.matches_rule_inner(s, rule);
            match result {
                RuleParseResult::Matches { leftover } => {
                    println!(
                        "s:{} matches rule: {} with leftovers: {}",
                        s, rule_id, leftover
                    );
                }
                RuleParseResult::DoesNotMatch => {
                    println!("s:{} does not match rule: {}", s, rule_id);
                }
            };
            result
        } else {
            self.matches_rule_inner(s, rule)
        }
    }

    fn matches_rule_inner<'a>(&self, s: &'a str, rule: &Rule) -> RuleParseResult<'a> {
        match rule {
            Rule::LiteralChar(char) => {
                if let Some(leftover) = s.strip_prefix(*char) {
                    RuleParseResult::Matches { leftover }
                } else {
                    RuleParseResult::DoesNotMatch
                }
            }
            Rule::RuleLists(alternatives) => {
                let matched = alternatives.iter().enumerate().find_map(|(i, rules)| {
                    if let RuleParseResult::Matches { leftover } = self.matches_rule_list(s, rules)
                    {
                        Some((i, leftover))
                    } else {
                        None
                    }
                });

                match matched {
                    Some((index_matched, leftover)) => {
                        // Special case: can we consume more of this leftover by exploring one of the other alternatives?
                        let matched_list = &alternatives[index_matched];

                        let remaining_alteratives: Vec<Vec<usize>> = alternatives
                            .iter()
                            .enumerate()
                            .filter_map(
                                |(i, rule)| {
                                    if i != index_matched {
                                        Some(rule)
                                    } else {
                                        None
                                    }
                                },
                            )
                            .filter_map(move |rule_list| {
                                // the alternative would need to begin with the same items as matched_list
                                if rule_list.starts_with(&matched_list) {
                                    // Create a virtual rule that represents the remaining slice of this alternative
                                    let new_rule_list = rule_list[matched_list.len()..].to_vec();
                                    Some(new_rule_list)
                                } else {
                                    None
                                }
                            })
                            .collect();

                        let extra_leftover_used = {
                            if remaining_alteratives.len() == 0 {
                                None
                            } else {
                                let virtual_alternative_rule =
                                    Rule::RuleLists(remaining_alteratives);

                                if let RuleParseResult::Matches { leftover } =
                                    self.matches_rule(leftover, &virtual_alternative_rule)
                                {
                                    Some(leftover)
                                } else {
                                    None
                                }
                            }
                        };

                        RuleParseResult::Matches {
                            leftover: extra_leftover_used.unwrap_or(leftover),
                        }
                    }
                    None => RuleParseResult::DoesNotMatch,
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
        // } else if s.contains(" | ") {
        //     let options: anyhow::Result<Vec<Rule>> = s.split(" | ").map(|x| x.parse()).collect();
        //     Ok(Rule::Alternatives(options?))
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

    // #[test]
    // fn answer() {}
}
