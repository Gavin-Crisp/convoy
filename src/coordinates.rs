use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Coord {
    pub rank: u8,
    pub file: u8,
}

impl Coord {
    #[must_use]
    pub const fn new(rank: u8, file: u8) -> Self {
        Self { rank, file }
    }

    /// Returns the distance between two coordinates in tiles through orthogonal connections
    ///
    /// # Example
    ///
    /// ```
    /// use convoy::coordinates::Coord;
    ///
    /// let here = Coord::new(0, 3);
    /// let there = Coord::new(1, 1);
    ///
    /// assert_eq!(here.distance(there), 3);
    /// ```
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
