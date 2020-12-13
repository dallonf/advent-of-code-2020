// Day 13: Shuttle Search

use shared::prelude::*;

#[derive(Debug)]
pub enum ScheduleEntry {
    X,
    Bus(u16),
}

#[derive(Debug)]
pub struct Input {
    timestamp: i64,
    schedule: Vec<ScheduleEntry>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct EarliestBusOutput {
    pub bus_id: u16,
    pub wait_time: i64,
}

lazy_static! {
    static ref TEST_INPUT: Input = Input::parse(&vec!["939", "7,13,x,x,59,x,31,19"]).unwrap();
    static ref PUZZLE_INPUT: Input =
        Input::parse(&puzzle_input::lines(include_str!("puzzle_input.txt"))).unwrap();
}

impl Input {
    pub fn parse(s: &[&str]) -> anyhow::Result<Input> {
        if s.len() != 2 {
            return Err(anyhow!("Input must be exactly two lines long"));
        }

        let timestamp = s[0].parse()?;
        let schedule = s[1]
            .split(",")
            .map(|x| {
                if x == "x" {
                    return Ok(ScheduleEntry::X);
                }
                Ok(ScheduleEntry::Bus(x.parse()?))
            })
            .collect::<anyhow::Result<_>>()?;

        Ok(Input {
            timestamp,
            schedule,
        })
    }

    pub fn earliest_bus(&self) -> Option<EarliestBusOutput> {
        self.schedule
            .iter()
            .filter_map(|x| {
                if let ScheduleEntry::Bus(bus_id) = x {
                    let bus_id = *bus_id;
                    let last_stop_number = self.timestamp / bus_id as i64;
                    let next_stop_time = (last_stop_number + 1) * bus_id as i64;
                    Some(EarliestBusOutput {
                        bus_id,
                        wait_time: next_stop_time - self.timestamp,
                    })
                } else {
                    None
                }
            })
            .min_by_key(|x| x.wait_time)
    }
}

#[cfg(test)]
mod part_one {
    use super::*;

    #[test]
    fn test_cases() {
        let result = TEST_INPUT.earliest_bus().unwrap();
        assert_eq!(
            result,
            EarliestBusOutput {
                bus_id: 59,
                wait_time: 5,
            }
        );
        assert_eq!(i64::from(result.bus_id) * result.wait_time, 295);
    }

    #[test]
    fn answer() {
        let result = PUZZLE_INPUT.earliest_bus().unwrap();
        assert_eq!(i64::from(result.bus_id) * result.wait_time, 4808);
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
