// Day 13: Shuttle Search

use shared::prelude::*;

#[derive(Debug, Copy, Clone)]
pub enum ScheduleEntry {
    X,
    Bus(u16),
}

#[derive(Debug, Clone)]
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

pub fn parse_bus_schedule(s: &str) -> anyhow::Result<Vec<ScheduleEntry>> {
    s.split(",")
        .map(|x| {
            if x == "x" {
                return Ok(ScheduleEntry::X);
            }
            Ok(ScheduleEntry::Bus(x.parse()?))
        })
        .collect::<anyhow::Result<_>>()
}

impl Input {
    pub fn parse(s: &[&str]) -> anyhow::Result<Input> {
        if s.len() != 2 {
            return Err(anyhow!("Input must be exactly two lines long"));
        }

        let timestamp = s[0].parse()?;
        let schedule = parse_bus_schedule(s[1])?;

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

fn all_equal<T: Eq>(arr: &[T]) -> bool {
    if arr.is_empty() {
        return true;
    }
    let first = &arr[0];
    arr.iter().all(|item| item == first)
}

pub fn earliest_sequence(input: &[ScheduleEntry]) -> i64 {
    let (mut times, buses): (Vec<i64>, Vec<i64>) = input
        .iter()
        .enumerate()
        .filter_map(|(i, x)| match x {
            ScheduleEntry::Bus(id) => Some((i, id)),
            ScheduleEntry::X => None,
        })
        .map(|(offset, bus_id)| (offset as i64 * -1, *bus_id as i64))
        .unzip();

    while !all_equal(&times) {
        let (smallest_index, value) = times
            .iter()
            .enumerate()
            .min_by_key(|(_, &time)| time)
            .unwrap();

        times[smallest_index] = value + buses[smallest_index];
    }

    times[0]
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

#[cfg(test)]
mod part_two {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(earliest_sequence(&TEST_INPUT.schedule), 1068781);
    }

    #[test]
    fn more_test_cases() {
        assert_eq!(
            earliest_sequence(&parse_bus_schedule("17,x,13,19").unwrap()),
            3417
        );
        assert_eq!(
            earliest_sequence(&parse_bus_schedule("67,7,59,61").unwrap()),
            754018
        );
        assert_eq!(
            earliest_sequence(&parse_bus_schedule("67,x,7,59,61").unwrap()),
            779210
        );
        assert_eq!(
            earliest_sequence(&parse_bus_schedule("67,7,x,59,61").unwrap()),
            1261476
        );
        assert_eq!(
            earliest_sequence(&parse_bus_schedule("1789,37,47,1889").unwrap()),
            1202161486
        );
    }

    #[test]
    fn answer() {
        assert_eq!(earliest_sequence(&PUZZLE_INPUT.schedule), 0);
    }
}
