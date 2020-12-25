// Day 25: Combo Breaker

use rayon::prelude::*;

pub struct Input {
    card_public_key: u64,
    door_public_key: u64,
}

pub const TEST_INPUT: Input = Input {
    card_public_key: 5764801,
    door_public_key: 17807724,
};

pub const PUZZLE_INPUT: Input = Input {
    card_public_key: 9789649,
    door_public_key: 3647239,
};

fn transform_step(number: u64, subject: u64) -> u64 {
    (number * subject) % 20201227
}

pub fn transform(subject: u64, loop_size: u64) -> u64 {
    let mut number = 1;
    for _ in 0..loop_size {
        number = transform_step(number, subject);
    }
    number
}

pub fn discover_loop_size(public_key: u64) -> u64 {
    let mut number = 1;
    let mut loop_size = 1;
    loop {
        number = transform_step(number, 7);
        if number == public_key {
            return loop_size;
        }
        loop_size += 1;
    }
}

pub fn discover_encryption_key(input: Input) -> u64 {
    vec![
        (input.door_public_key, input.card_public_key),
        (input.card_public_key, input.door_public_key),
    ]
    .into_par_iter()
    .find_map_any(|(public_key, other_public_key)| {
        let loop_size = discover_loop_size(public_key);
        Some(transform(other_public_key, loop_size))
    })
    .unwrap()
}

#[cfg(test)]
mod part_one {
    use super::*;

    #[test]
    fn test_transform() {
        assert_eq!(transform(TEST_INPUT.door_public_key, 8), 14897079);
        assert_eq!(transform(TEST_INPUT.card_public_key, 11), 14897079);
    }

    #[test]
    fn test_discover_loop_size_1() {
        assert_eq!(discover_loop_size(TEST_INPUT.card_public_key), 8);
    }

    #[test]
    fn test_discover_loop_size_2() {
        assert_eq!(discover_loop_size(TEST_INPUT.door_public_key), 11);
    }

    #[test]
    fn test_discover_encryption_key() {
        // do it a bunch of times to make sure
        // the parallelization doesn't cause instability
        assert!((0..10)
            .into_par_iter()
            .all(|_| discover_encryption_key(TEST_INPUT) == 14897079));
    }

    #[test]
    fn answer() {
        assert_eq!(discover_encryption_key(PUZZLE_INPUT), 8740494);
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
