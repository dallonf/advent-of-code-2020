// Day 20: Jurassic Jigsaw

use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    iter,
};

// use rayon::prelude::*;
use shared::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Tile {
    tile_id: u64,
    size: usize,
    data: Vec<bool>,
}

#[derive(Debug, PartialEq, Eq, Hash, Default, Clone, Copy)]
pub struct Transformation {
    flip_x: bool,
    flip_y: bool,
    rotated: bool,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Image {
    size: usize,
    remaining_tiles: Vec<Tile>,
    grid: Vec<Option<TilePlacement>>,
}

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct TilePlacement(Tile, Transformation);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Direction {
    Up,
    Right,
    Left,
    Down,
}

type EdgeMap = HashMap<Vec<bool>, Vec<(TilePlacement, Direction)>>;

lazy_static! {
    static ref TILE_REGEX: Regex = Regex::new(r"^Tile ([0-9]+):$").unwrap();
    static ref TEST_INPUT: Vec<Tile> =
        parse_input(&puzzle_input::lines(include_str!("test_input.txt"))).unwrap();
    static ref PUZZLE_INPUT: Vec<Tile> =
        parse_input(&puzzle_input::lines(include_str!("puzzle_input.txt"))).unwrap();
}

pub fn parse_input(input: &[&str]) -> anyhow::Result<Vec<Tile>> {
    input
        .split(|line| line.is_empty())
        .map(|section| Tile::parse(section))
        .collect()
}

impl Tile {
    pub fn parse(section: &[&str]) -> anyhow::Result<Tile> {
        let first_line = section.get(0).ok_or(anyhow!("empty section"))?;
        let tile_id = TILE_REGEX
            .captures(first_line)
            .ok_or(anyhow!("invalid header line"))
            .and_then(|captures| captures[1].parse::<u64>().map_err(|err| err.into()))?;

        let size = section.len() - 1;

        if !section.iter().skip(1).all(|line| line.len() == size) {
            return Err(anyhow!("Tile isn't square"));
        }

        let data = section
            .iter()
            .skip(1)
            .flat_map(|line| line.chars().map(|char| char == '#'))
            .collect();

        Ok(Tile {
            tile_id,
            size,
            data,
        })
    }
}

impl Transformation {
    pub fn all() -> impl Iterator<Item = Transformation> {
        let booleans = || vec![true, false].into_iter();
        booleans().flat_map(move |rotated| {
            booleans().flat_map(move |flip_x| {
                booleans().map(move |flip_y| Transformation {
                    rotated,
                    flip_x,
                    flip_y,
                })
            })
        })
    }
}

impl Image {
    fn all_edges(tiles: &[Tile]) -> EdgeMap {
        tiles
    .iter()
    .flat_map(|tile| {
        Transformation::all().flat_map(move |transform| {
            let placement = TilePlacement(tile.clone(), transform);
            Direction::all().map(move |direction| {
                let edge = placement.edge(direction);
                (edge, placement.clone(), direction)
            })
        })
    })
    .fold(
        HashMap::new(),
        |mut result: EdgeMap,
         (edge, placement, direction): (Vec<bool>, TilePlacement, Direction)| {
            if !result.contains_key(&edge) {
                result.insert(edge.clone(), vec![]);
            }

            let vec = result.get_mut(&edge).unwrap();
            vec.push((placement, direction));

            result
        },
    )
    }

    fn new(tiles: &[Tile]) -> anyhow::Result<Image> {
        let size = (tiles.len() as f64).sqrt().floor();
        if size.fract() > 0.001 {
            return Err(anyhow!("Tiles do not fit into a square"));
        }
        let size = size as usize;

        Ok(Image {
            size,
            grid: iter::repeat(None).take(tiles.len()).collect(),
            remaining_tiles: tiles.to_vec(),
        })
    }

    fn fill(&self, edges_map: &EdgeMap) -> Vec<Image> {
        let first_unfilled_tile = self.grid.iter().enumerate().find_map(|(i, placement)| {
            if let None = placement {
                Some(i)
            } else {
                None
            }
        });

        let first_unfilled_tile = if let Some(x) = first_unfilled_tile {
            x
        } else {
            return vec![self.clone()];
        };

        let (x, y) = self.index_to_coord(first_unfilled_tile);

        let matches = self.matches_for(edges_map, x, y);

        let next_images = matches.into_iter().map(|placement| Image {
            size: self.size,
            grid: {
                let mut x = self.grid.clone();
                x[first_unfilled_tile] = Some(placement.clone());
                x
            },
            remaining_tiles: {
                let x = self
                    .remaining_tiles
                    .iter()
                    .filter(|&tile| tile != &placement.0)
                    .cloned()
                    .collect();
                x
            },
        });

        next_images
            .flat_map(|image| image.fill(edges_map))
            .collect()
    }

    fn matches_for(&self, edges_map: &EdgeMap, x: usize, y: usize) -> Vec<TilePlacement> {
        let edge_above = if y != 0 {
            self.tile_at(x, y - 1).map(|tile| tile.bottom_edge())
        } else {
            None
        };
        let edge_to_left = if x != 0 {
            self.tile_at(x - 1, y).map(|tile| tile.right_edge())
        } else {
            None
        };

        let possible_matches_above: Option<HashSet<&TilePlacement>> =
            edge_above.and_then(|edge_above| {
                edges_map.get(&edge_above).map(|results| {
                    results
                        .iter()
                        .filter_map(|(placement, direction)| {
                            if *direction == Direction::Up {
                                Some(placement)
                            } else {
                                None
                            }
                        })
                        .collect()
                })
            });

        let possible_matches_to_left: Option<HashSet<&TilePlacement>> =
            edge_to_left.and_then(|edge_to_left| {
                edges_map.get(&edge_to_left).map(|results| {
                    results
                        .iter()
                        .filter_map(|(placement, direction)| {
                            if *direction == Direction::Left {
                                Some(placement)
                            } else {
                                None
                            }
                        })
                        .collect()
                })
            });

        let possible_matches: Vec<TilePlacement> =
            match (possible_matches_above, possible_matches_to_left) {
                (None, None) => self
                    .remaining_tiles
                    .iter()
                    .flat_map(|tile| {
                        Transformation::all().map(move |transformation| {
                            TilePlacement(tile.to_owned(), transformation)
                        })
                    })
                    .collect(),
                (None, Some(set)) | (Some(set), None) => set.into_iter().cloned().collect(),
                (Some(up_set), Some(left_set)) => {
                    up_set.intersection(&left_set).map(|&x| x.clone()).collect()
                }
            };

        // Should only consider the tiles still in our library to place
        possible_matches
            .into_iter()
            .filter(|TilePlacement(tile, _)| self.remaining_tiles.contains(tile))
            .collect()
    }

    fn tile_at(&self, x: usize, y: usize) -> Option<&TilePlacement> {
        if x > self.size {
            return None;
        }

        let index = y * self.size + x;
        if index >= self.grid.len() {
            return None;
        }

        (&self.grid[index]).into()
    }

    fn index_to_coord(&self, index: usize) -> (usize, usize) {
        let x = index % self.size;
        let y = index / self.size;
        (x, y)
    }
}

impl TilePlacement {
    fn right_edge(&self) -> Vec<bool> {
        let size = self.0.size;
        (0..size).map(|y| self.pixel_at(size - 1, y)).collect()
    }

    fn left_edge(&self) -> Vec<bool> {
        let size = self.0.size;
        (0..size).map(|y| self.pixel_at(0, y)).collect()
    }

    fn top_edge(&self) -> Vec<bool> {
        let size = self.0.size;
        (0..size).map(|x| self.pixel_at(x, 0)).collect()
    }

    fn bottom_edge(&self) -> Vec<bool> {
        let size = self.0.size;
        (0..size).map(|x| self.pixel_at(x, size - 1)).collect()
    }

    fn edge(&self, direction: Direction) -> Vec<bool> {
        match direction {
            Direction::Up => self.top_edge(),
            Direction::Right => self.right_edge(),
            Direction::Left => self.left_edge(),
            Direction::Down => self.bottom_edge(),
        }
    }

    fn pixel_at(&self, mut x: usize, mut y: usize) -> bool {
        let size = self.0.size;
        let transformation = self.1;
        if transformation.flip_x {
            x = (size - 1) - x;
        }
        if transformation.flip_y {
            y = (size - 1) - y;
        }
        if transformation.rotated {
            let (new_x, new_y) = ((size - 1) - y, x);
            x = new_x;
            y = new_y;
        }

        let index = y * size + x;
        self.0.data[index]
    }

    pub fn display_tile(&self) -> String {
        (0..self.0.size)
            .map(|y| {
                (0..self.0.size)
                    .map(|x| if self.pixel_at(x, y) { "#" } else { "." })
                    .collect::<Vec<&str>>()
                    .concat()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl Debug for TilePlacement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TilePlacement")
            .field("tile_id", &self.0.tile_id)
            .field("transformation", &self.1)
            .finish()
            .and_then(|()| write!(f, "\n{}\n", self.display_tile()))
    }
}

impl Direction {
    pub fn all() -> impl Iterator<Item = Direction> {
        vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .into_iter()
    }
}

pub fn get_corner_ids(tiles: &[Tile]) -> anyhow::Result<HashSet<u64>> {
    let image = Image::new(tiles)?;

    let final_image = image
        .fill(&Image::all_edges(tiles))
        .into_iter()
        .find(|_| true);

    final_image
        .ok_or(anyhow!("Solution not found"))
        .map(|final_image| {
            vec![
                final_image.tile_at(0, 0),
                final_image.tile_at(final_image.size - 1, 0),
                final_image.tile_at(0, final_image.size - 1),
                final_image.tile_at(final_image.size - 1, final_image.size - 1),
            ]
            .into_iter()
            .map(|x| match x {
                Some(TilePlacement(tile, _)) => tile.tile_id,
                None => panic!("corners should be filled in"),
            })
            .collect()
        })
}

#[cfg(test)]
mod part_one {
    use super::*;

    fn edge_to_string(edge: &[bool]) -> String {
        edge.iter()
            .map(|x| if *x { "#" } else { "." })
            .collect::<Vec<_>>()
            .concat()
    }

    #[test]
    fn test_edges() {
        let test_tile = TEST_INPUT.iter().find(|x| x.tile_id == 2311).unwrap();
        let placement = TilePlacement(test_tile.clone(), Transformation::default());

        assert_eq!(edge_to_string(&placement.top_edge()), "..##.#..#.");
        assert_eq!(edge_to_string(&placement.bottom_edge()), "..###..###");
        assert_eq!(edge_to_string(&placement.left_edge()), ".#####..#.");
        assert_eq!(edge_to_string(&placement.right_edge()), "...#.##..#");
    }

    #[test]
    fn test_selection() {
        let edge_map = Image::all_edges(TEST_INPUT.as_slice());
        let mut test_image = Image::new(TEST_INPUT.as_slice()).unwrap();

        let test_tile = TEST_INPUT
            .iter()
            .find(|x| x.tile_id == 1951)
            .unwrap()
            .clone();

        test_image.grid[0] = Some(TilePlacement(
            test_tile.clone(),
            Transformation {
                flip_y: true,
                ..Default::default()
            },
        ));

        test_image.remaining_tiles = test_image
            .remaining_tiles
            .into_iter()
            .filter(|tile| tile != &test_tile)
            .collect();

        let matches = test_image.matches_for(&edge_map, 1, 0);

        assert_eq!(matches.len(), 1);
        assert!(matches.iter().any(|x| {
            x.0.tile_id == 2311 && x.1.flip_x == false && x.1.flip_y == true && x.1.rotated == false
        }));
    }

    #[test]
    fn test_case() {
        let result = get_corner_ids(TEST_INPUT.as_slice()).unwrap();

        assert_eq!(result, vec![1951, 3079, 2971, 1171].into_iter().collect());
        assert_eq!(result.iter().product::<u64>(), 20899048083289);
    }

    #[test]
    fn answer() {
        let result = get_corner_ids(PUZZLE_INPUT.as_slice()).unwrap();
        assert_eq!(result.iter().product::<u64>(), 54755174472007);
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
