// Day 00: Template

use std::{collections::HashMap, ops::Range};

use shared::prelude::*;

pub type Rules = HashMap<String, Vec<Range<u32>>>;

pub type Ticket = Vec<u32>;

#[derive(Default, Debug, PartialEq, Eq)]
pub struct ProblemNotes {
    rules: Rules,
    your_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

lazy_static! {
    static ref RULE_REGEX: Regex = Regex::new(r"^([a-z ]+): ([0-9]+)\-([0-9]+) or ([0-9]+)\-([0-9]+)$").unwrap();

    static ref TEST_INPUT: ProblemNotes = ProblemNotes::parse_input(&puzzle_input::lines(include_str!("test_input.txt"))).unwrap();
    // static ref PUZZLE_INPUT: Vec<&'static str> =
    //     puzzle_input::lines(include_str!("puzzle_input.txt"));
}

impl ProblemNotes {
    pub fn parse_input(lines: &[&str]) -> anyhow::Result<ProblemNotes> {
        let mut sections = lines.split(|x| x.is_empty());
        let rules_section = sections.next().ok_or(anyhow!("Missing rules section"))?;
        let your_ticket_section = sections
            .next()
            .ok_or(anyhow!("Missing your ticket section"))?;
        let nearby_tickets_section = sections
            .next()
            .ok_or(anyhow!("Missing nearby tickets section"))?;

        let rules = rules_section
            .into_iter()
            .copied()
            .map(|x| -> anyhow::Result<(String, Vec<Range<u32>>)> {
                let captures = RULE_REGEX
                    .captures(x)
                    .ok_or(anyhow!("Rule doesn't match format"))?;
                let field_name = captures[1].to_string();

                let (low1, high1, low2, high2) =
                    (&captures[2], &captures[3], &captures[4], &captures[5]);

                let range1 = low1.parse().unwrap()..high1.parse().unwrap();
                let range2 = low2.parse().unwrap()..high2.parse().unwrap();

                Ok((field_name, vec![range1, range2]))
            })
            .collect::<anyhow::Result<Rules>>()?;

        let your_ticket = {
            if your_ticket_section.len() != 2 || your_ticket_section.get(0) != Some(&"your ticket:")
            {
                Err(anyhow!("your ticket section improperly formatted"))
            } else {
                your_ticket_section[1]
                    .split(",")
                    .map(|x| x.parse().map_err(|x| anyhow::Error::from(x)))
                    .collect()
            }
        }?;

        let nearby_tickets = {
            if nearby_tickets_section.get(0) != Some(&"nearby tickets:") {
                Err(anyhow!("nearby tickets sections improperly formatted"))
            } else {
                nearby_tickets_section
                    .into_iter()
                    .skip(1)
                    .map(|x| {
                        x.split(",")
                            .map(|y| y.parse::<u32>().map_err(|y| anyhow::Error::from(y)))
                            .collect()
                    })
                    .collect()
            }
        }?;

        Ok(ProblemNotes {
            rules,
            your_ticket,
            nearby_tickets,
        })
    }
}

#[cfg(test)]
mod part_one {
    use super::*;

    #[test]
    fn test_parsing() {
        assert_eq!(TEST_INPUT.rules.len(), 3);
        assert_eq!(TEST_INPUT.your_ticket.len(), 3);
        assert_eq!(TEST_INPUT.nearby_tickets.len(), 4);
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
