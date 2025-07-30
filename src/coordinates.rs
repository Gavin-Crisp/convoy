use crate::board::Board;
use std::fmt::Debug;

pub trait Coordinate: Copy + Debug + Default + Eq + PartialEq {
    fn rank(self) -> u8;
    fn file(self) -> u8;
    fn distance(self, other: impl Coordinate) -> u8;
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Coord {
    rank: u8,
    file: u8,
}

impl Coord {
    #[must_use]
    pub const fn new(rank: u8, file: u8) -> Self {
        Self { rank, file }
    }

    #[must_use]
    pub const fn adjacent(self) -> [Self; 4] {
        [
            Self {
                rank: self.rank + 1,
                file: self.file,
            },
            Self {
                rank: self.rank - 1,
                file: self.file,
            },
            Self {
                rank: self.rank,
                file: self.file + 1,
            },
            Self {
                rank: self.rank,
                file: self.file - 1,
            },
        ]
    }
}

impl Coordinate for Coord {
    #[must_use]
    fn rank(self) -> u8 {
        self.rank
    }

    #[must_use]
    fn file(self) -> u8 {
        self.file
    }

    /// Returns the distance between two coordinates in tiles through orthogonal connections
    #[must_use]
    fn distance(self, other: impl Coordinate) -> u8 {
        self.rank.abs_diff(other.rank()) + self.file.abs_diff(other.file())
    }
}

/// A `TileCoord` will always refer to a tile
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct TileCoord(Coord);

impl TileCoord {
    #[must_use]
    pub fn new(rank: u8, file: u8, board: &Board) -> Option<Self> {
        Self::new_from_coord(Coord::new(rank, file), board)
    }

    #[must_use]
    pub fn new_from_coord(coord: Coord, board: &Board) -> Option<Self> {
        board.get(coord)?;
        Some(Self(coord))
    }

    #[must_use]
    pub const fn as_coord(self) -> Coord {
        self.0
    }
}

impl Coordinate for TileCoord {
    fn rank(self) -> u8 {
        self.0.rank
    }

    fn file(self) -> u8 {
        self.0.file
    }

    fn distance(self, other: impl Coordinate) -> u8 {
        self.0.distance(other)
    }
}
/// A `PieceCoord` will always refer to a tile with a piece on it
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct PieceCoord(Coord);

impl PieceCoord {
    #[must_use]
    pub fn new(rank: u8, file: u8, board: &Board) -> Option<Self> {
        Self::new_from_coord(Coord::new(rank, file), board)
    }

    #[must_use]
    pub fn new_from_coord(coord: Coord, board: &Board) -> Option<Self> {
        board.get(coord)?.piece_option?;
        Some(Self(coord))
    }

    #[must_use]
    pub const fn as_coord(self) -> Coord {
        self.0
    }

    #[must_use]
    pub const fn as_tile_coord(self) -> TileCoord {
        TileCoord(self.0)
    }
}

impl Coordinate for PieceCoord {
    fn rank(self) -> u8 {
        self.0.rank
    }

    fn file(self) -> u8 {
        self.0.file
    }

    fn distance(self, other: impl Coordinate) -> u8 {
        self.0.distance(other)
    }
}
