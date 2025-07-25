use crate::{coordinates::Coord, tile::Tile};
use std::ops::{Index, IndexMut};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Board {
    /// Vec of ranks of tiles
    map: Vec<Option<Tile>>,
    ranks: u8,
    files: u8,
}

impl Board {
    // TODO: Better new fn
    #[must_use]
    pub const fn new() -> Self {
        Self {
            map: vec![],
            ranks: 0,
            files: 0,
        }
    }

    #[must_use]
    pub const fn ranks(&self) -> u8 {
        self.ranks
    }

    #[must_use]
    pub const fn files(&self) -> u8 {
        self.files
    }

    #[must_use]
    pub const fn new_coord(&self, rank: u8, file: u8) -> Option<Coord> {
        if rank <= self.ranks && file <= self.files {
            Some(Coord { rank, file })
        } else {
            None
        }
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
            self.map[(coord.rank * self.files + coord.file) as usize]
        }
    }

    /// Returns a mutable reference to the tile at `coord` or an error
    ///
    /// # Errors
    ///
    /// `coord` must index a tile that exists.
    pub fn get_mut(&mut self, coord: Coord) -> Option<&mut Tile> {
        if self.coord_in_bounds(coord) {
            self.map[(coord.rank * self.files + coord.file) as usize].as_mut()
        } else {
            None
        }
    }

    pub fn tiles(&self) -> impl Iterator<Item = Tile> {
        self.map.iter().filter_map(|tile_option| *tile_option)
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
                coord + Coord { rank: 1, file: 0 },
                coord - Coord { rank: 1, file: 0 },
                coord + Coord { rank: 0, file: 1 },
                coord - Coord { rank: 0, file: 1 },
            ]
            .into_iter()
            .filter_map(|coord| self.get(coord).map(|_| coord))
            .collect(),
        )
    }

    pub fn piece_coords(&self) -> impl Iterator<Item = Coord> {
        (0..self.ranks)
            .flat_map(|rank| (0..self.files).map(move |file| Coord { rank, file }))
            .zip(self.map.iter())
            .filter_map(|(coord, tile_option)| {
                if tile_option.is_some_and(|tile| tile.piece_option.is_some()) {
                    Some(coord)
                } else {
                    None
                }
            })
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Index<Coord> for Board {
    type Output = Tile;

    fn index(&self, index: Coord) -> &Self::Output {
        self.map[(index.rank * self.files + index.file) as usize]
            .as_ref()
            .expect("Indexed invalid tile")
    }
}

impl IndexMut<Coord> for Board {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        self.map[(index.rank * self.files + index.file) as usize]
            .as_mut()
            .expect("Indexed invalid tile")
    }
}
