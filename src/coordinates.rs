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

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Move {
    pub from: Coord,
    pub to: Coord,
}

impl Move {
    #[must_use]
    pub const fn new(from: Coord, to: Coord) -> Self {
        Self { from, to }
    }

    /// Returns the distance between the start and end position
    ///
    /// # Example
    ///
    /// ```
    /// use convoy::coordinates::{Coord, Move};
    ///
    /// let r#move = Move::new(Coord::new(2, 3), Coord::new(5, 2));
    ///
    /// assert_eq!(r#move.distance(), 4);
    /// ```
    #[must_use]
    pub const fn distance(self) -> u8 {
        self.from.distance(self.to)
    }
}
