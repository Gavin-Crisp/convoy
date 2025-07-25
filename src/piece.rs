use crate::player::Player;
use std::ops::Range;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Piece {
    kind: PieceType,
    exhausted: bool,
    owner: Player,
}

impl Piece {
    #[must_use]
    pub const fn kind(self) -> PieceType {
        self.kind
    }

    #[must_use]
    pub const fn exhausted(self) -> bool {
        self.exhausted
    }

    #[must_use]
    pub const fn owner(self) -> Player {
        self.owner
    }

    #[must_use]
    pub const fn speed(self) -> u8 {
        match self.kind {
            PieceType::Artillery | PieceType::Infantry => 2,
            PieceType::Convoy | PieceType::Recon => 3,
        }
    }

    #[must_use]
    pub const fn power(self) -> u8 {
        match self.kind {
            PieceType::Artillery | PieceType::Infantry => 2,
            PieceType::Convoy => 0,
            PieceType::Recon => 1,
        }
    }

    #[must_use]
    pub const fn range(self) -> Range<u8> {
        match self.kind {
            PieceType::Artillery => 2..4,
            PieceType::Convoy => 0..0,
            PieceType::Infantry | PieceType::Recon => 1..2,
        }
    }

    #[must_use]
    pub const fn cost(self) -> u8 {
        match self.kind {
            PieceType::Artillery | PieceType::Convoy => 3,
            PieceType::Infantry => 2,
            PieceType::Recon => 4,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PieceType {
    Artillery,
    Convoy,
    Infantry,
    Recon,
}
