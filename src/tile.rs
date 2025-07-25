use crate::{piece::Piece, Player};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Tile {
    pub kind: TileType,
    pub piece_option: Option<Piece>,
}

impl Tile {
    #[must_use]
    pub const fn income_bonus(self) -> u8 {
        match self.kind {
            TileType::Empty | TileType::Border(_) => 0,
            TileType::Town => 1,
            TileType::City => 3,
        }
    }

    #[must_use]
    pub const fn defence_bonus(self) -> u8 {
        match self.kind {
            TileType::Empty | TileType::Border(_) => 0,
            TileType::Town => 1,
            TileType::City => 2,
        }
    }

    #[must_use]
    pub fn can_recruit(self, player: Player) -> bool {
        match self.kind {
            TileType::Empty | TileType::Town | TileType::City => false,
            TileType::Border(controller) => player == controller,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TileType {
    Empty,
    Town,
    City,
    Border(Player),
}
