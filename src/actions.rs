use crate::{coordinates::Coord, piece::PieceType};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Command {
    Move {
        from: Coord,
        to: Coord,
    },
    Recruit {
        piece_type: PieceType,
        coord: Coord,
    },
    Attack {
        piece: Coord,
        target: Coord,
    },
    MoveAttack {
        from: Coord,
        to: Coord,
        target: Coord,
    },
}
