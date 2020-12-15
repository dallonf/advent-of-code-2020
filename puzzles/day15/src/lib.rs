// Day 15: Rambunctious Recitation

use std::collections::HashMap;

use shared::prelude::*;

lazy_static! {
    static ref PUZZLE_INPUT: Vec<u32> = vec![5, 2, 8, 16, 18, 0, 1];
}

pub fn result_of_turn(starting: &[u32], final_turn_number: u32) -> u32 {
    let starting_turns = starting.len() - 1;
    let mut memory: HashMap<u32, u32> = starting
        .iter()
        .copied()
        .take(starting_turns) // we'll play the last starting number directly
        .enumerate()
        .map(|(i, x)| (x, i as u32))
        .collect();

    let mut prev_number = starting[starting_turns];
    for turn_index in starting_turns as u32 + 1..final_turn_number {
        let last_turn_of_prev_number = memory.get(&prev_number);
        let turns_since_repeat = if let Some(last_turn_of_prev_number) = last_turn_of_prev_number {
            (turn_index - 1) - last_turn_of_prev_number
        } else {
            0
        };

        memory.insert(prev_number, turn_index - 1);
        prev_number = turns_since_repeat;
    }

    prev_number
}

#[cfg(test)]
mod part_one {
    use super::*;

    #[test]
    fn basics() {
        assert_eq!(result_of_turn(&vec![0, 3, 6], 9), 4);
    }

    #[test]
    fn test_case() {
        assert_eq!(result_of_turn(&vec![0, 3, 6], 2020), 436);
    }

    #[test]
    fn more_test_cases() {
        assert_eq!(result_of_turn(&vec![1, 3, 2], 2020), 1);
        assert_eq!(result_of_turn(&vec![2, 1, 3], 2020), 10);
        assert_eq!(result_of_turn(&vec![1, 2, 3], 2020), 27);
        assert_eq!(result_of_turn(&vec![2, 3, 1], 2020), 78);
        assert_eq!(result_of_turn(&vec![3, 2, 1], 2020), 438);
        assert_eq!(result_of_turn(&vec![3, 1, 2], 2020), 1836);
    }

    #[test]
    fn answer() {
        assert_eq!(result_of_turn(PUZZLE_INPUT.as_slice(), 2020), 517);
    }
}

#[cfg(test)]
mod part_two {
    use super::*;
    #[test]
    fn test_case() {
        assert_eq!(result_of_turn(&vec![0, 3, 6], 30_000_000), 175594);
    }

    #[test]
    fn answer() {
        assert_eq!(result_of_turn(PUZZLE_INPUT.as_slice(), 30_000_000), 1047739);
    }
}
