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

    #[must_use]
    pub const fn can_initiate(self) -> bool {
        self.kind.can_initiate()
    }

    #[must_use]
    pub const fn can_defend(self) -> bool {
        self.kind.can_defend()
    }

    #[must_use]
    pub const fn can_support(self, is_attacking: bool, is_moving: bool) -> bool {
        self.kind.can_support(is_attacking, is_moving)
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

    #[must_use]
    pub const fn can_initiate(self) -> bool {
        matches!(self, Self::Infantry | Self::Recon)
    }

    #[must_use]
    pub const fn can_defend(self) -> bool {
        matches!(self, Self::Infantry | Self::Recon)
    }

    #[must_use]
    pub const fn can_support(self, is_attacking: bool, is_moving: bool) -> bool {
        match self {
            Self::Artillery => !is_moving,
            Self::Convoy => false,
            Self::Infantry => !is_moving || is_attacking,
            Self::Recon => true,
        }
    }
}
