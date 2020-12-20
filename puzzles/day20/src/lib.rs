// Day 20: Jurassic Jigsaw

use std::{collections::HashSet, iter};

use rayon::prelude::*;
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
    rotation: u8, // 0-3
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Image {
    size: usize,
    remaining_tiles: Vec<Tile>,
    grid: Vec<Option<TilePlacement>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TilePlacement(Tile, Transformation);

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
    pub fn all() -> impl ParallelIterator<Item = Transformation> {
        let booleans = || vec![true, false].into_par_iter();
        (0..4_u8).into_par_iter().flat_map(move |rotation| {
            booleans().flat_map(move |flip_x| {
                booleans().map(move |flip_y| Transformation {
                    rotation,
                    flip_x,
                    flip_y,
                })
            })
        })
    }
}

impl Image {
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

    fn fill(&self) -> Vec<Image> {
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

        let possibilities = self
            .remaining_tiles
            .par_iter()
            .enumerate()
            .flat_map(|(i, tile)| {
                Transformation::all()
                    .map(move |transformation| (i, TilePlacement(tile.to_owned(), transformation)))
            });

        let verified_possibilities =
            possibilities.filter(|(_, placement)| self.placement_would_fit(placement, x, y));

        let next_images = verified_possibilities.map(|(i, placement)| Image {
            size: self.size,
            grid: {
                let mut x = self.grid.clone();
                x[first_unfilled_tile] = Some(placement);
                x
            },
            remaining_tiles: {
                let mut x = self.remaining_tiles.clone();
                x.remove(i);
                x
            },
        });

        next_images.flat_map(|image| image.fill()).collect()
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

    fn placement_would_fit(&self, placement: &TilePlacement, x: usize, y: usize) -> bool {
        let would_fit_left = {
            if x == 0 {
                true
            } else {
                let left_tile = self.tile_at(x - 1, y);
                left_tile
                    .map(|left_tile| left_tile.right_edge() == placement.left_edge())
                    .unwrap_or(true)
            }
        };

        let would_fit_right = {
            let right_tile = self.tile_at(x + 1, y);
            right_tile
                .map(|right_tile| right_tile.left_edge() == placement.right_edge())
                .unwrap_or(true)
        };

        let would_fit_up = {
            if y == 0 {
                true
            } else {
                let up_tile = self.tile_at(x, y - 1);
                up_tile
                    .map(|up_tile| up_tile.bottom_edge() == placement.top_edge())
                    .unwrap_or(true)
            }
        };

        let would_fit_down = {
            let down_tile = self.tile_at(x, y + 1);
            down_tile
                .map(|down_tile| down_tile.top_edge() == placement.bottom_edge())
                .unwrap_or(true)
        };

        would_fit_left && would_fit_right && would_fit_up && would_fit_down
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

    fn pixel_at(&self, mut x: usize, mut y: usize) -> bool {
        let size = self.0.size;
        let transformation = self.1;
        if transformation.flip_x {
            x = (size - 1) - x;
        }
        if transformation.flip_y {
            y = (size - 1) - y;
        }

        for _ in 0..transformation.rotation {
            let (new_x, new_y) = ((size - 1) - y, x);
            x = new_x;
            y = new_y;
        }

        let index = y * size + x;

        self.0.data[index]
    }
}

pub fn get_corner_ids(tiles: &[Tile]) -> anyhow::Result<HashSet<u64>> {
    let image = Image::new(tiles)?;

    let final_image = image.fill().into_iter().find(|_| true);

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
    fn test_case() {
        let result = get_corner_ids(TEST_INPUT.as_slice()).unwrap();

        assert_eq!(result, vec![1951, 3079, 2971, 1171].into_iter().collect());
        assert_eq!(result.iter().product::<u64>(), 20899048083289);
    }

    #[test]
    fn answer() {
        let result = get_corner_ids(PUZZLE_INPUT.as_slice()).unwrap();
        assert_eq!(result.iter().product::<u64>(), 0);
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
