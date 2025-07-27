use crate::{coordinates::Coord, piece::PieceType};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Command {
    Move {
        from: Coord,
        to: Coord,
    },
    Recruit {
        piece_type: PieceType,
        coord: Coord,
    },
    Battle {
        target: Coord,
        target_is_defending: bool,
        initiator: BattleActor,
        attack_supporters: Vec<BattleActor>,
        defence_supporters: Vec<BattleActor>,
    },
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BattleActor {
    Static { coord: Coord },
    Moving { from: Coord, to: Coord },
}
