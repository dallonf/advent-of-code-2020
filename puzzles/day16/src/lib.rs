// Day 00: Template

use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};

use shared::prelude::*;

pub type Rules = HashMap<String, Vec<Range<u32>>>;

pub type Ticket = Vec<u32>;

#[derive(Default, Debug, PartialEq, Eq)]
pub struct FieldMapping(Vec<String>);

#[derive(Default, Debug, PartialEq, Eq)]
pub struct ProblemNotes {
    rules: Rules,
    your_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

lazy_static! {
    static ref RULE_REGEX: Regex =
        Regex::new(r"^([a-z ]+): ([0-9]+)\-([0-9]+) or ([0-9]+)\-([0-9]+)$").unwrap();
    static ref TEST_INPUT: ProblemNotes =
        ProblemNotes::parse_input(&puzzle_input::lines(include_str!("test_input.txt"))).unwrap();
    static ref TEST_INPUT_2: ProblemNotes =
        ProblemNotes::parse_input(&puzzle_input::lines(include_str!("test_input_2.txt"))).unwrap();
    static ref PUZZLE_INPUT: ProblemNotes =
        ProblemNotes::parse_input(&puzzle_input::lines(include_str!("puzzle_input.txt"))).unwrap();
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

                // Note that Rust ranges are exclusive at the top range, but the input ranges are formatted as inclusive
                let range1 = low1.parse().unwrap()..high1.parse::<u32>().unwrap() + 1;
                let range2 = low2.parse().unwrap()..high2.parse::<u32>().unwrap() + 1;

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

    pub fn scanning_error_rate(&self) -> u32 {
        let all_ranges: Vec<&Range<u32>> = self.rules.values().flatten().collect();

        self.nearby_tickets
            .iter()
            .flatten()
            .filter(|x| !all_ranges.iter().any(|range| range.contains(x)))
            .sum()
    }

    pub fn field_mapping(&self) -> FieldMapping {
        let all_valid_tickets: Vec<&Ticket> = {
            let all_ranges: Vec<&Range<u32>> = self.rules.values().flatten().collect();

            let not_invalid = |ticket: &&Ticket| {
                ticket
                    .iter()
                    .all(|x| all_ranges.iter().any(|range| range.contains(x)))
            };

            self.nearby_tickets
                .iter()
                .filter(not_invalid)
                .chain(std::iter::once(&self.your_ticket))
                .collect()
        };

        let columns: Vec<Vec<u32>> = {
            let num_columns = self.rules.len();

            let all_values_for_column =
                |i| all_valid_tickets.iter().map(|ticket| ticket[i]).collect();

            (0..num_columns).map(all_values_for_column).collect()
        };

        let possibilities_for_columns: Vec<HashSet<&String>> = {
            let all_possibilities: HashSet<&String> = self.rules.keys().collect();

            let possibilities_for_column = |column: &Vec<u32>| {
                column.iter().fold(all_possibilities.clone(), |prev, num| {
                    let matches_rule = |key: &&String| {
                        self.rules
                            .get(key.to_owned())
                            .unwrap()
                            .iter()
                            .any(|range| range.contains(num))
                    };

                    prev.into_iter().filter(matches_rule).collect()
                })
            };

            columns.iter().map(possibilities_for_column).collect()
        };

        #[derive(Debug, PartialEq, Eq, Clone)]
        enum ColumnState<'a> {
            Solved(&'a String),
            Possibilities(HashSet<&'a String>),
        }

        fn solve_columns(possibilities: Vec<ColumnState>) -> FieldMapping {
            // All are solved; stop recursing
            if possibilities.iter().all(|x| {
                if let ColumnState::Solved(_) = x {
                    true
                } else {
                    false
                }
            }) {
                return FieldMapping(
                    possibilities
                        .into_iter()
                        .map(|x| {
                            if let ColumnState::Solved(x) = x {
                                x.to_owned()
                            } else {
                                panic!()
                            }
                        })
                        .collect(),
                );
            }

            let all_solved: HashSet<&String> = possibilities
                .iter()
                .filter_map(|x| {
                    if let ColumnState::Solved(x) = x {
                        Some(x)
                    } else {
                        None
                    }
                })
                .copied()
                .collect();

            let possibilities_after_solving: Vec<ColumnState> = {
                possibilities
                    .iter()
                    .enumerate()
                    .map(|(i, x)| match x {
                        ColumnState::Solved(x) => ColumnState::Solved(x),
                        ColumnState::Possibilities(column_possibilities) => {
                            let column_possibilities: HashSet<&String> = column_possibilities
                                .difference(&all_solved)
                                .copied()
                                .collect();

                            let all_other_possibilities: HashSet<&String> = possibilities
                                .iter()
                                .enumerate()
                                .map(|(j, x)| {
                                    if j == i {
                                        return None;
                                    }
                                    if let ColumnState::Possibilities(x) = x {
                                        Some(x)
                                    } else {
                                        None
                                    }
                                })
                                .flatten()
                                .fold(HashSet::new(), |a, b| a.union(b).copied().collect());

                            let unique_column_possibilities: HashSet<&String> =
                                column_possibilities
                                    .difference(&all_other_possibilities)
                                    .copied()
                                    .collect();

                            if unique_column_possibilities.len() == 1 {
                                ColumnState::Solved(
                                    unique_column_possibilities.iter().next().unwrap(),
                                )
                            } else {
                                ColumnState::Possibilities(column_possibilities)
                            }
                        }
                    })
                    .collect()
            };

            if possibilities_after_solving == possibilities {
                panic!("No solutions found! Got stuck on: {:#?}", possibilities);
            }

            solve_columns(possibilities_after_solving)
        }

        solve_columns(
            possibilities_for_columns
                .into_iter()
                .map(|x| ColumnState::Possibilities(x))
                .collect(),
        )
    }
}

impl FieldMapping {
    pub fn translate(&self, ticket: &Ticket) -> HashMap<String, u32> {
        self.0
            .iter()
            .enumerate()
            .map(|(i, key)| (key.to_owned(), ticket[i]))
            .collect()
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

    #[test]
    fn test_case() {
        assert_eq!(TEST_INPUT.scanning_error_rate(), 71);
    }

    #[test]
    fn answer() {
        assert_eq!(PUZZLE_INPUT.scanning_error_rate(), 25961);
    }
}

#[cfg(test)]
mod part_two {
    use super::*;
    #[test]
    fn test_case() {
        let mapping = TEST_INPUT_2.field_mapping();

        let mapped_ticket = mapping.translate(&TEST_INPUT_2.your_ticket);

        let expected: HashMap<String, u32> = vec![
            ("class".to_string(), 12),
            ("row".to_string(), 11),
            ("seat".to_string(), 13),
        ]
        .into_iter()
        .collect();

        assert_eq!(mapped_ticket, expected);
    }

    #[test]
    fn answer() {
        let mapped_ticket = PUZZLE_INPUT
            .field_mapping()
            .translate(&PUZZLE_INPUT.your_ticket);

        let result: u64 = mapped_ticket
            .iter()
            .filter(|(key, _)| key.starts_with("departure "))
            .map(|(_, &value)| u64::from(value))
            .product();

        assert_eq!(result, 603409823791);
    }
}
