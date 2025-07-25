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
    pub const fn new(kind: PieceType, owner: Player) -> Self {
        Self {
            kind,
            exhausted: true,
            owner,
        }
    }

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
        self.kind.speed()
    }

    #[must_use]
    pub const fn power(self) -> u8 {
        self.kind.power()
    }

    #[must_use]
    pub const fn range(self) -> Range<u8> {
        self.kind.range()
    }

    #[must_use]
    pub const fn cost(self) -> u8 {
        self.kind.cost()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PieceType {
    Artillery,
    Convoy,
    Infantry,
    Recon,
}

impl PieceType {
    #[must_use]
    pub const fn speed(self) -> u8 {
        match self {
            Self::Artillery | Self::Infantry => 2,
            Self::Convoy | Self::Recon => 3,
        }
    }

    #[must_use]
    pub const fn power(self) -> u8 {
        match self {
            Self::Artillery | Self::Infantry => 2,
            Self::Convoy => 0,
            Self::Recon => 1,
        }
    }

    #[must_use]
    pub const fn range(self) -> Range<u8> {
        match self {
            Self::Artillery => 2..4,
            Self::Convoy => 0..0,
            Self::Infantry | Self::Recon => 1..2,
        }
    }

    #[must_use]
    pub const fn cost(self) -> u8 {
        match self {
            Self::Artillery | Self::Convoy => 3,
            Self::Infantry => 2,
            Self::Recon => 4,
        }
    }
}
