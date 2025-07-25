use crate::{
    actions::{BattleActor, Command},
    board::Board,
    coordinates::Coord,
    piece::PieceType,
    player::Player,
};

pub mod actions;
pub mod board;
pub mod coordinates;
pub mod piece;
pub mod player;
pub mod tile;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Game {
    board: Board,
    current_player: Player,
}

impl Game {
    #[must_use]
    pub const fn board(&self) -> &Board {
        &self.board
    }

    #[must_use]
    pub const fn current_player(&self) -> Player {
        self.current_player
    }
}

impl Game {
    pub fn do_command(&mut self, command: Command) {
        match command {
            Command::Move { from, to } => self.do_move(from, to),
            Command::Recruit { piece_type, coord } => self.do_recruit(piece_type, coord),
            Command::Battle {
                target,
                initiator,
                attack_supporters,
                defence_supporters,
            } => self.do_battle(target, initiator, attack_supporters, defence_supporters),
        }
    }

    pub const fn do_move(&mut self, _from: Coord, _to: Coord) {
        todo!()
    }

    pub const fn do_recruit(&mut self, _piece_type: PieceType, _coord: Coord) {
        todo!()
    }

    pub fn do_battle(
        &mut self,
        _target: Coord,
        _initiator: BattleActor,
        _attack_supporters: Vec<BattleActor>,
        _defence_supporters: Vec<BattleActor>,
    ) {
        todo!()
    }
}
