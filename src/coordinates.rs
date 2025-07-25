use std::{
    fmt::{Display, Formatter},
    ops::{Add, AddAssign, Sub, SubAssign},
};

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Coord {
    pub(crate) rank: u8,
    pub(crate) file: u8,
}

impl Coord {
    #[must_use]
    pub const fn rank(self) -> u8 {
        self.rank
    }

    #[must_use]
    pub const fn file(self) -> u8 {
        self.file
    }

    /// Returns the distance between two coordinates in tiles through orthogonal connections
    #[must_use]
    pub const fn distance(self, other: Self) -> u8 {
        self.rank.abs_diff(other.rank) + self.file.abs_diff(other.file)
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}:{})", self.rank, self.file)
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            rank: self.rank.wrapping_add(rhs.rank),
            file: self.file.wrapping_add(rhs.file),
        }
    }
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            rank: self.rank.wrapping_sub(rhs.rank),
            file: self.file.wrapping_sub(rhs.file),
        }
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        self.rank += rhs.rank;
        self.file += rhs.file;
    }
}

impl SubAssign for Coord {
    fn sub_assign(&mut self, rhs: Self) {
        self.rank -= rhs.rank;
        self.file -= rhs.file;
    }
}
