// Day 00: Template

use std::str::FromStr;

use shared::prelude::*;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Default)]
pub struct Differences {
    one_jolt: usize,
    three_jolt: usize,
}

lazy_static! {
    static ref PUZZLE_INPUT: Vec<u16> = puzzle_input::lines(include_str!("puzzle_input.txt"))
        .into_iter()
        .map(FromStr::from_str)
        .collect::<Result<_, _>>()
        .unwrap();
}

pub fn get_differences(adapters: &[u16]) -> anyhow::Result<Differences> {
    let sorted = {
        let mut sorted = adapters.to_owned();
        sorted.sort();
        sorted
    };

    let device_adapter = sorted.last().unwrap_or(&0) + 3;

    sorted
        .iter()
        .chain(std::iter::once(&device_adapter))
        .scan(0, |prev, next| {
            let difference = *next - *prev;
            *prev = *next;
            Some(difference)
        })
        .try_fold(Differences::default(), |mut result, difference| {
            match difference {
                1 => {
                    result.one_jolt += 1;
                }
                3 => {
                    result.three_jolt += 1;
                }
                other => return Err(anyhow!("Unsupported difference: {}", other)),
            };
            Ok(result)
        })
}

#[cfg(test)]
mod part_one {
    use super::*;

    #[test]
    fn small_test_case() {
        assert_eq!(
            get_differences(&vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]).unwrap(),
            Differences {
                one_jolt: 7,
                three_jolt: 5
            }
        );
    }

    #[test]
    fn large_test_case() {
        assert_eq!(
            get_differences(&vec![
                28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25,
                35, 8, 17, 7, 9, 4, 2, 34, 10, 3
            ])
            .unwrap(),
            Differences {
                one_jolt: 22,
                three_jolt: 10
            }
        );
    }

    #[test]
    fn answer() {
        let result = get_differences(PUZZLE_INPUT.as_slice()).unwrap();
        assert_eq!(result.one_jolt * result.three_jolt, 1625);
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
