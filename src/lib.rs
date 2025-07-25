use crate::{
    actions::{BattleActor, Command},
    board::Board,
    coordinates::Coord,
    piece::{Piece, PieceType},
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
    money: [u8; 2],
}

impl Game {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            board: Board::new(),
            current_player: Player::P1,
            money: [0, 0],
        }
    }

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
    pub fn do_command(&mut self, command: &Command) {
        match command {
            Command::Move { from, to } => self.do_move(*from, *to),
            Command::Recruit { piece_type, coord } => self.do_recruit(*piece_type, *coord),
            Command::Battle {
                target,
                initiator,
                attack_supporters,
                defence_supporters,
            } => self.do_battle(*target, *initiator, attack_supporters, defence_supporters),
        }
    }

    pub fn do_move(&mut self, from: Coord, to: Coord) {
        if !self.can_move(from, to, self.current_player) {
            // TODO: Error handling
            return;
        }

        self.board.move_piece(from, to);
    }

    #[must_use]
    pub fn can_move(&self, from: Coord, to: Coord, player: Player) -> bool {
        let Some(from_tile) = self.board.get(from) else {
            // TODO: Error handling
            return false;
        };

        let Some(to_tile) = self.board.get(to) else {
            // TODO: Error handling
            return false;
        };

        if to_tile.piece_option.is_some() {
            // TODO: Error handling
            return false;
        }

        let Some(piece) = from_tile.piece_option else {
            // TODO: Error handling
            return false;
        };

        if piece.exhausted {
            // TODO: Error handling
            return false;
        }

        if piece.owner() != player {
            return false;
        }

        if from.distance(to) > piece.speed() {
            // TODO: Error handling
            return false;
        }

        // TODO: check pathing

        true
    }

    pub fn do_recruit(&mut self, piece_type: PieceType, coord: Coord) {
        let Some(tile) = self.board.get(coord) else {
            // TODO: Error handling
            return;
        };

        if tile.piece_option.is_some() {
            // TODO: Error handling
            return;
        }

        if !tile.can_recruit(self.current_player) {
            // TODO: Error handling
            return;
        }

        if self.money[self.current_player] < piece_type.cost() {
            // TODO: Error handling
            return;
        }

        self.money[self.current_player] -= piece_type.cost();
        self.board[coord].piece_option = Some(Piece::new(piece_type, self.current_player));
    }

    pub fn do_battle(
        &mut self,
        _target: Coord,
        _initiator: BattleActor,
        _attack_supporters: &[BattleActor],
        _defence_supporters: &[BattleActor],
    ) {
        todo!()
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
