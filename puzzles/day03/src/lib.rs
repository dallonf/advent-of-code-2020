// Day 3: Toboggan Trajectory

use shared::prelude::*;

lazy_static! {
    static ref PUZZLE_INPUT: Vec<String> = puzzle_input::lines(include_str!("puzzle_input.txt"));
}

#[cfg(test)]
mod part_one {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(1 + 1, 2);
    }

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
