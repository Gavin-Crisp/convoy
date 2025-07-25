use crate::{coordinates::Coord, tile::Tile};
use std::ops::{Index, IndexMut};

pub struct Board {
    /// Vec of ranks of tiles
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
            .flat_map(|rank| rank.iter().filter_map(|tile_option| *tile_option))
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

    #[expect(clippy::missing_panics_doc)]
    pub fn piece_coords(&self) -> impl Iterator<Item = Coord> {
        self.map.iter().enumerate().flat_map(|(i, rank)| {
            rank.iter().enumerate().filter_map(move |(j, tile_option)| {
                if tile_option.is_some_and(|tile| tile.piece_option.is_some()) {
                    Some(Coord {
                        rank: i.try_into().expect("Valid Boards are <= 256x256"),
                        file: j.try_into().expect("Valid Boards are <= 256x256"),
                    })
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
