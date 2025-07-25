use crate::{coordinates::Coord, tile::Tile};
use std::ops::{Index, IndexMut};

pub struct Board {
    map: Vec<Vec<Option<Tile>>>,
    ranks: u8,
    files: u8,
}

impl Board {
    #[must_use]
    pub const fn ranks(&self) -> u8 {
        self.ranks
    }

    #[must_use]
    pub const fn files(&self) -> u8 {
        self.files
    }

    #[must_use]
    pub const fn coord_in_bounds(&self, coord: Coord) -> bool {
        coord.rank < self.ranks && coord.file < self.files
    }

    /// Returns a copy of the tile at `coord` or an error
    ///
    /// # Errors
    ///
    /// `coord` must index a tile that exists.
    #[must_use]
    pub fn get(&self, coord: Coord) -> Option<Tile> {
        if coord.rank >= self.ranks || coord.file >= self.files {
            None
        } else {
            self.map[coord.rank as usize][coord.file as usize]
        }
    }

    /// Returns a mutable reference to the tile at `coord` or an error
    ///
    /// # Errors
    ///
    /// `coord` must index a tile that exists.
    pub fn get_mut(&mut self, coord: Coord) -> Option<&mut Tile> {
        if self.coord_in_bounds(coord) {
            self.map[coord.rank as usize][coord.file as usize].as_mut()
        } else {
            None
        }
    }

    pub fn tiles(&self) -> impl Iterator<Item = Tile> {
        self.map
            .iter()
            .flat_map(|rank| rank.iter().filter_map(|t| *t))
    }

    /// Returns a Vec of the coords of all neighbours of `coord` or an error
    ///
    /// # Errors
    ///
    /// `coord` must index a tile that exists.
    #[must_use]
    pub fn coord_neighbours(&self, coord: Coord) -> Option<Vec<Coord>> {
        if !self.coord_in_bounds(coord) {
            return None;
        }

        Some(
            [
                coord + Coord::new(1, 0),
                coord - Coord::new(1, 0),
                coord + Coord::new(0, 1),
                coord - Coord::new(0, 1),
            ]
            .into_iter()
            .filter_map(|coord| self.get(coord).map(|_| coord))
            .collect(),
        )
    }

    #[expect(clippy::missing_panics_doc)]
    pub fn piece_coords(&self) -> impl Iterator<Item = Coord> {
        self.map.iter().enumerate().flat_map(|(i, rank)| {
            rank.iter().enumerate().filter_map(move |(j, t)| {
                if t.is_some_and(|tile| tile.piece_option.is_some()) {
                    Some(Coord::new(
                        i.try_into().expect("Valid Boards are <= 256x256"),
                        j.try_into().expect("Valid Boards are <= 256x256"),
                    ))
                } else {
                    None
                }
            })
        })
    }
}

impl Index<Coord> for Board {
    type Output = Tile;

    fn index(&self, index: Coord) -> &Self::Output {
        self.map[index.rank as usize][index.file as usize]
            .as_ref()
            .expect("Indexed invalid tile")
    }
}

impl IndexMut<Coord> for Board {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        self.map[index.rank as usize][index.file as usize]
            .as_mut()
            .expect("Indexed invalid tile")
    }
}
