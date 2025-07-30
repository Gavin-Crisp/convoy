use crate::{
    coordinates::{PieceCoord, TileCoord},
    piece::PieceType,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Command {
    Move {
        from: PieceCoord,
        to: TileCoord,
    },
    Recruit {
        piece_type: PieceType,
        coord: TileCoord,
    },
    Battle {
        target: PieceCoord,
        target_is_defending: bool,
        initiator: BattleActor,
        attack_supporters: Vec<BattleActor>,
        defence_supporters: Vec<BattleActor>,
    },
    EndTurn,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BattleActor {
    Static { coord: PieceCoord },
    Moving { from: PieceCoord, to: TileCoord },
}
