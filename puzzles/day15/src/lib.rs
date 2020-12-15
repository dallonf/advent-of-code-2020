// Day 15: Rambunctious Recitation

use std::collections::HashMap;

use shared::prelude::*;

lazy_static! {
    static ref PUZZLE_INPUT: Vec<u32> = vec![5, 2, 8, 16, 18, 0, 1];
}

pub fn result_of_turn(starting: &[u32], turn_number: u32) -> u32 {
    let starting_turns = starting.len() - 1;
    let memory: HashMap<u32, u32> = starting
        .iter()
        .copied()
        .take(starting_turns) // we'll play the last starting number directly
        .enumerate()
        .map(|(i, x)| (x, starting_turns as u32 - i as u32))
        .collect();

    let (final_number, _) = (starting_turns as u32 + 1..turn_number).fold(
        (starting[starting_turns], memory),
        |(last_number, mut memory), _| {
            let turns_since_repeat = *memory.get(&last_number).unwrap_or(&0);
            memory.insert(last_number, 0); // it has been 0 turns since this number

            let next_memory = memory
                .into_iter()
                .map(|(number, age)| (number, age + 1))
                .collect();

            (turns_since_repeat, next_memory)
        },
    );

    final_number
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
    fn answer() {
        assert_eq!(result_of_turn(PUZZLE_INPUT.as_slice(), 2020), 517);
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
