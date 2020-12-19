// Day 19: Monster Messages

use std::{collections::HashMap, str::FromStr, todo};

use shared::prelude::*;

#[derive(Debug)]
pub struct Input<'a>(Rules, Vec<&'a str>);

#[derive(Debug, Clone)]
pub struct Rules {
    rule_map: HashMap<usize, Rule>,
}

#[derive(Debug, Clone)]
pub enum Rule {
    LiteralChar(char),
    RuleList(Vec<usize>),
    Alternatives(Vec<Rule>),
}

lazy_static! {
    static ref PUZZLE_INPUT: Input<'static> =
        Input::parse(&puzzle_input::lines(include_str!("puzzle_input.txt"))).unwrap();
    static ref TEST_INPUT: Input<'static> =
        Input::parse(&puzzle_input::lines(include_str!("test_input.txt"))).unwrap();
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
    static ref RULE_REGEX: Regex = Regex::new(r"^([0-9]): (.+)$").unwrap();
    static ref LITERAL_CHAR_REGEX: Regex = Regex::new(r#"^"([a-z])"$"#).unwrap();
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

        Ok(Rules { rule_map: rule_map? })
    }

    pub fn matches(&self, s: &str) -> bool {
        todo!()
    }
}

impl FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(literal_char_captures) = LITERAL_CHAR_REGEX.captures(s) {
            Ok(Rule::LiteralChar(
                literal_char_captures[1].chars().nth(0).unwrap(),
            ))
        } else if s.contains(" | ") {
            let options: anyhow::Result<Vec<Rule>> = s.split(" | ").map(|x| x.parse()).collect();
            Ok(Rule::Alternatives(options?))
        } else {
            let ids: Result<Vec<usize>, _> = s
                .split(" ")
                .map(|x| x.parse().map_err(|e| anyhow::Error::from(e)))
                .collect();
            Ok(Rule::RuleList(ids?))
        }
    }
}

#[cfg(test)]
mod part_one {
    use super::*;

    #[test]
    fn test_cases() {
        let Input(rules, values) = TEST_INPUT.deref();
        println!("{:#?}", rules);

        let matches: Vec<&str> = values
            .iter()
            .copied()
            .filter(|x| rules.matches(x))
            .collect();

        assert_eq!(matches, vec!["ababbb", "abbbab"]);
    }

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
