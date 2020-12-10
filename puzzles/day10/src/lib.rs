// Day 00: Template

use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};
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

fn hash_slice(slice: &[u16]) -> u64 {
    let mut hasher = DefaultHasher::new();
    slice.hash(&mut hasher);
    hasher.finish()
}

fn get_valid_combinations(adapters: &[u16]) -> u64 {
    let sorted = {
        let mut sorted = adapters.to_owned();
        sorted.sort();
        sorted
    };

    let device_adapter = sorted.last().unwrap_or(&0) + 3;
    let full_collection: Vec<u16> = std::iter::once(0)
        .chain(sorted.iter().copied())
        .chain(std::iter::once(device_adapter))
        .collect();

    fn slice_valid_combinations(slice: &[u16], cache: &mut HashMap<u64, u64>) -> u64 {
        if slice.len() <= 0 {
            return 1;
        }
        if let Some(result) = cache.get(&hash_slice(slice)) {
            return *result;
        }

        let current = slice[0];
        let max_next = current + 3;
        

        println!("🥽 {:?}", slice);
        let valid_next_options = slice
            .iter()
            .copied()
            .enumerate()
            .skip(1)
            .inspect(|x| println!("👑 {:?} - {}", x, x.1 <= max_next))
            .take_while(|(_i, x)| *x <= max_next);

        // println!("🥽 {:?}", valid_next_options.clone().collect::<Vec<_>>());

        // TODO: actually use the cache

        valid_next_options
            .map(|(i, _)| slice_valid_combinations(&slice[i..], cache))
            .sum()
    }

    slice_valid_combinations(&full_collection, &mut HashMap::new())
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

#[cfg(test)]
mod part_two {
    use super::*;
    #[test]
    fn small_test_case() {
        assert_eq!(
            get_valid_combinations(&vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]),
            8
        );
    }
    // #[test]
    // fn answer() {}
}
