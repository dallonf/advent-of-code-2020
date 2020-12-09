// Day 9: Encoding Error

use std::str::FromStr;

use shared::prelude::*;

#[derive(Debug)]
pub struct XmasStream<'a> {
    prev: &'a [i64],
    next: &'a i64,
}

#[derive(Debug)]
pub enum XmasStreamResult {
    Continue,
    Invalid(i64),
}

lazy_static! {
    static ref PUZZLE_INPUT: Vec<i64> = puzzle_input::lines(include_str!("puzzle_input.txt"))
        .into_iter()
        .map(FromStr::from_str)
        .collect::<Result<_, _>>()
        .unwrap();
    static ref TEST_INPUT: Vec<i64> = vec![
        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576
    ];
}

impl XmasStream<'_> {
    fn result(&self) -> XmasStreamResult {
        let mut possible_matches =
            self.prev
                .iter()
                .enumerate()
                .flat_map(|(i, x)| -> Box<dyn Iterator<Item = i64>> {
                    if i + 1 >= self.prev.len() {
                        return Box::new(std::iter::empty());
                    }
                    let others = &self.prev[i + 1..];
                    Box::new(others.iter().map(move |y| x + y))
                });

        if possible_matches.any(|x| &x == self.next) {
            XmasStreamResult::Continue
        } else {
            XmasStreamResult::Invalid(*self.next)
        }
    }
}

pub fn find_first_invalid_number(stream: &[i64], preamble: usize) -> Option<i64> {
    let stream_points = stream
        .iter()
        .enumerate()
        .skip(preamble)
        .map(|(i, x)| XmasStream {
            prev: &stream[i - preamble..i],
            next: x,
        });

    stream_points.map(|x| x.result()).find_map(|x| match x {
        XmasStreamResult::Invalid(x) => Some(x),
        _ => None,
    })
}

pub fn find_encryption_weakness(stream: &[i64], preamble: usize) -> Option<i64> {
    let target = find_first_invalid_number(stream, preamble)?;

    let weak_range = (0..stream.len()).map(|i| &stream[i..]).find_map(|slice| {
        slice
            .iter()
            .enumerate()
            .scan(0, |sum, (i, x)| {
                *sum += x;
                if *sum == target {
                    Some(Some(&slice[..i])) // The range we're looking for
                } else if *sum > target {
                    None // Stop iteration
                } else {
                    Some(None) // Continue
                }
            })
            .find_map(|x| x)
    })?;

    Some(weak_range.iter().min()? + weak_range.iter().max()?)
}

#[cfg(test)]
mod part_one {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            find_first_invalid_number(TEST_INPUT.as_slice(), 5).unwrap(),
            127
        );
    }

    #[test]
    fn answer() {
        assert_eq!(
            find_first_invalid_number(PUZZLE_INPUT.as_slice(), 25).unwrap(),
            1212510616
        );
    }
}

#[cfg(test)]
mod part_two {
    use super::*;
    #[test]
    fn test_case() {
        assert_eq!(
            find_encryption_weakness(TEST_INPUT.as_slice(), 5).unwrap(),
            62
        );
    }

    #[test]
    fn answer() {
        assert_eq!(
            find_encryption_weakness(PUZZLE_INPUT.as_slice(), 25).unwrap(),
            171265123
        );
    }
}
