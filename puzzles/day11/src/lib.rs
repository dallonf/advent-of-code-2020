// Day 11: Seating System

use std::{convert::TryFrom, convert::TryInto, fmt::Debug, str::FromStr};

use shared::prelude::*;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum SeatState {
    Floor,
    Empty,
    Occupied,
}

#[derive(Eq, PartialEq, Clone)]
pub struct SeatLayout {
    seats: Vec<SeatState>,
    row_len: usize,
}

lazy_static! {
    static ref TEST_INPUT: SeatLayout = include_str!("test_input.txt").parse().unwrap();
    static ref PUZZLE_INPUT: SeatLayout = include_str!("puzzle_input.txt").parse().unwrap();
}

impl FromStr for SeatLayout {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = puzzle_input::lines(s);

        let rows: Vec<Vec<SeatState>> = lines
            .into_iter()
            .map(|l| {
                l.chars()
                    .map(|char| match char {
                        '.' => Ok(SeatState::Floor),
                        'L' => Ok(SeatState::Empty),
                        '#' => Ok(SeatState::Occupied),
                        other => Err(anyhow!("Unrecognized char code: {}", other)),
                    })
                    .collect::<Result<_, _>>()
            })
            .collect::<anyhow::Result<_>>()?;

        let row_len = rows[0].len();
        let seats: Vec<SeatState> = rows.iter().flatten().cloned().collect();

        Ok(SeatLayout { seats, row_len })
    }
}

impl Debug for SeatLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = (0..(self.seats.len() / self.row_len))
            .map(move |y| -> String {
                (0..self.row_len)
                    .map(move |x| {
                        let coordinate = (x, y);
                        let seat = self.seat_at(&coordinate);
                        match seat {
                            SeatState::Floor => '.',
                            SeatState::Empty => 'L',
                            SeatState::Occupied => '#',
                        }
                    })
                    .collect()
            })
            .collect::<Vec<_>>()
            .join("\n");

        f.write_str(&result)
    }
}

impl SeatLayout {
    fn map(&self, f: impl Fn(&(usize, usize), &SeatState) -> SeatState + Copy) -> SeatLayout {
        let new_seats = (0..(self.seats.len() / self.row_len)).flat_map(move |y| {
            (0..self.row_len).map(move |x| {
                let coordinate = (x, y);
                let seat = self.seat_at(&coordinate);
                f(&coordinate, seat)
            })
        });

        SeatLayout {
            row_len: self.row_len,
            seats: new_seats.collect(),
        }
    }

    fn seat_at(&self, (x, y): &(usize, usize)) -> &SeatState {
        let index = y * self.row_len + x;
        return &self.seats[index];
    }

    fn adjacent_seats(&self, (x, y): &(usize, usize)) -> impl Iterator<Item = &SeatState> {
        let directions = {
            let max_x = self.row_len - 1;
            let max_y = (self.seats.len() / self.row_len) - 1;
            let x_left = if x <= &0 { None } else { Some(x - 1_usize) };
            let x_right = if x >= &max_x { None } else { Some(x + 1_usize) };
            let y_up = if y <= &0 { None } else { Some(y - 1_usize) };
            let y_down = if y >= &max_y { None } else { Some(y + 1_usize) };
            let x = Some(*x);
            let y = Some(*y);
            vec![
                (x, y_up),         // up
                (x_right, y_up),   // up-right
                (x_right, y),      // right
                (x_right, y_down), // down-right
                (x, y_down),       // down
                (x_left, y_down),  // down-left
                (x_left, y),       // left
                (x_left, y_up),    // up-left
            ]
        };

        directions.into_iter().filter_map(move |(x, y)| {
            if let (Some(x), Some(y)) = (x, y) {
                Some(self.seat_at(&(x, y)))
            } else {
                None
            }
        })
    }

    fn visible_occupied_seats_from(&self, (x, y): &(usize, usize)) -> usize {
        let directions: Vec<(i32, i32)> = (-1..2)
            .flat_map(|x| (-1..2).map(move |y| (x, y)))
            .filter(|x| x != &(0, 0)) // make sure 0,0 isn't in the list of directions!!
            .collect();

        let row_len: i32 = self.row_len.try_into().unwrap();
        let max_x: i32 = row_len - 1;
        let max_y: i32 = (i32::try_from(self.seats.len()).unwrap() / row_len) - 1;
        let x: i32 = x.to_owned().try_into().unwrap();
        let y: i32 = y.to_owned().try_into().unwrap();

        directions
            .into_iter()
            .map(|(dx, dy)| {
                struct Context<'a> {
                    dx: i32,
                    dy: i32,
                    max_x: i32,
                    max_y: i32,
                    layout: &'a SeatLayout,
                }

                fn look(x: i32, y: i32, ctx: &Context) -> i32 {
                    let (next_x, next_y) = (x + ctx.dx, y + ctx.dy);
                    if next_x < 0 || next_x > ctx.max_x || next_y < 0 || next_y > ctx.max_y {
                        0 // out of bounds; haven't found an occupied seat
                    } else {
                        let seat = ctx
                            .layout
                            .seat_at(&(next_x.try_into().unwrap(), next_y.try_into().unwrap()));

                        match seat {
                            SeatState::Empty => 0,
                            SeatState::Occupied => 1,
                            SeatState::Floor => look(next_x, next_y, ctx), // Keep looking
                        }
                    }
                };

                look(
                    x,
                    y,
                    &Context {
                        dx,
                        dy,
                        max_x,
                        max_y,
                        layout: self,
                    },
                )
            })
            .sum::<i32>()
            .try_into()
            .unwrap()
    }

    pub fn iterate(&self) -> SeatLayout {
        self.map(|coord, seat| {
            if seat == &SeatState::Floor {
                return *seat;
            }

            let adjacent_occupied = self
                .adjacent_seats(coord)
                .filter(|x| x == &&SeatState::Occupied)
                .count();

            if seat == &SeatState::Empty && adjacent_occupied == 0 {
                SeatState::Occupied
            } else if seat == &SeatState::Occupied && adjacent_occupied >= 4 {
                SeatState::Empty
            } else {
                *seat
            }
        })
    }

    pub fn iterate_mk2(&self) -> SeatLayout {
        self.map(|coord, seat| {
            if seat == &SeatState::Floor {
                return *seat;
            }

            let visible_occupied = self.visible_occupied_seats_from(coord);

            if seat == &SeatState::Empty && visible_occupied == 0 {
                SeatState::Occupied
            } else if seat == &SeatState::Occupied && visible_occupied >= 5 {
                SeatState::Empty
            } else {
                *seat
            }
        })
    }

    pub fn iterate_until_stable(&self) -> SeatLayout {
        let next_state = self.iterate();
        if &next_state == self {
            next_state
        } else {
            next_state.iterate_until_stable()
        }
    }

    pub fn iterate_until_stable_mk2(&self) -> SeatLayout {
        let next_state = self.iterate_mk2();
        if &next_state == self {
            next_state
        } else {
            next_state.iterate_until_stable_mk2()
        }
    }

    pub fn occupied(&self) -> usize {
        self.seats
            .iter()
            .filter(|x| x == &&SeatState::Occupied)
            .count()
    }
}

#[cfg(test)]
mod part_one {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(TEST_INPUT.iterate_until_stable().occupied(), 37);
    }

    #[test]
    fn answer() {
        assert_eq!(PUZZLE_INPUT.iterate_until_stable().occupied(), 2489);
    }
}

#[cfg(test)]
mod part_two {
    use super::*;

    #[test]
    fn visibility() {
        let test_layout: SeatLayout = r"
#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##
        "
        .parse()
        .unwrap();

        assert_eq!(test_layout.visible_occupied_seats_from(&(2, 0)), 5);
    }

    #[test]
    fn single_step() {
        let original: SeatLayout = r"
#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##
"
        .parse()
        .unwrap();

        let expected: SeatLayout = r"
#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#
"
        .parse()
        .unwrap();

        assert_eq!(original.iterate_mk2(), expected);
    }

    #[test]
    fn test_cases() {
        assert_eq!(TEST_INPUT.iterate_until_stable_mk2().occupied(), 26);
    }

    #[test]
    fn answer() {
        assert_eq!(PUZZLE_INPUT.iterate_until_stable_mk2().occupied(), 2180);
    }
}
