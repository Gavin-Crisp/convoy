use crate::{
    actions::{BattleActor, Command},
    board::Board,
    coordinates::{Coord, Coordinate},
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
                target_is_defending,
                initiator,
                attack_supporters,
                defence_supporters,
            } => self.do_battle(
                *target,
                *target_is_defending,
                *initiator,
                attack_supporters,
                defence_supporters,
            ),
            Command::EndTurn => self.do_end_turn(),
        }
    }

    #[expect(clippy::missing_panics_doc)]
    pub fn do_move(&mut self, from: Coord, to: Coord) {
        if !self.can_do_move(from, to) {
            // TODO: Error handling
            return;
        }

        let mut piece = self.board[from]
            .piece_option
            .expect("Already checked for error");

        piece.exhausted = true;
        self.board[to].piece_option = Some(piece);
        self.board[from].piece_option = None;
    }

    #[must_use]
    pub fn can_do_move(&self, from: Coord, to: Coord) -> bool {
        self.can_move(from, to, self.current_player)
    }

    #[must_use]
    fn can_move(&self, from: Coord, to: Coord, player: Player) -> bool {
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
        if !self.can_do_recruit(piece_type, coord) {
            // TODO: Error handling
            return;
        }

        self.money[self.current_player] -= piece_type.cost();
        self.board[coord].piece_option = Some(Piece::new(piece_type, self.current_player));
    }

    #[must_use]
    pub fn can_do_recruit(&self, piece_type: PieceType, coord: Coord) -> bool {
        let Some(tile) = self.board.get(coord) else {
            // TODO: Error handling
            return false;
        };

        if tile.piece_option.is_some() {
            // TODO: Error handling
            return false;
        }

        if !tile.can_recruit(self.current_player) {
            // TODO: Error handling
            return false;
        }

        if self.money[self.current_player] < piece_type.cost() {
            // TODO: Error handling
            return false;
        }

        true
    }

    pub fn do_battle(
        &mut self,
        target: Coord,
        target_is_defending: bool,
        initiator: BattleActor,
        attack_supporters: &[BattleActor],
        defence_supporters: &[BattleActor],
    ) {
        if !self.can_do_battle(
            target,
            target_is_defending,
            initiator,
            attack_supporters,
            defence_supporters,
        ) {
            // TODO: Error handling
            return;
        }

        let attack_power = attack_supporters
            .iter()
            .fold(self.resolve_actor(initiator), |power, actor| {
                power + self.resolve_actor(*actor)
            });
        let defence_power = defence_supporters.iter().fold(
            if target_is_defending {
                self.resolve_actor(BattleActor::Static { coord: target })
            } else {
                0
            },
            |power, actor| power + self.resolve_actor(*actor),
        );

        if attack_power > defence_power {
            self.board[target].piece_option = None;
        }
    }

    fn resolve_actor(&mut self, battle_actor: BattleActor) -> u8 {
        match battle_actor {
            BattleActor::Static { coord } => {
                let mut piece = self.board[coord]
                    .piece_option
                    .expect("Already checked for errors");
                piece.exhausted = true;
                self.board[coord].piece_option = Some(piece);
                piece.power()
            }
            BattleActor::Moving { from, to } => {
                let mut piece = self.board[from]
                    .piece_option
                    .expect("Already checked for errors");
                piece.exhausted = true;
                self.board[to].piece_option = Some(piece);
                self.board[from].piece_option = None;
                piece.power()
            }
        }
    }

    #[must_use]
    pub fn can_do_battle(
        &self,
        target: Coord,
        target_is_defending: bool,
        initiator: BattleActor,
        attack_supporters: &[BattleActor],
        defence_supporters: &[BattleActor],
    ) -> bool {
        let Some(target_tile) = self.board.get(target) else {
            // TODO: Error handling
            return false;
        };

        let Some(target_piece) = target_tile.piece_option else {
            // TODO: Error handling
            return false;
        };

        if target_piece.owner() == self.current_player {
            // TODO: Error handling
            return false;
        }

        if target_is_defending && !target_piece.can_defend() {
            // TODO: Error handling
            return false;
        }

        if !self.validate_actor(initiator, target, self.current_player, true) {
            // TODO: Error handling
            return false;
        }

        if !attack_supporters
            .iter()
            .all(|actor| self.validate_actor(*actor, target, self.current_player, true))
        {
            // TODO: Error handling
            return false;
        }

        if !defence_supporters
            .iter()
            .all(|actor| self.validate_actor(*actor, target, -self.current_player, false))
        {
            // TODO: Error handling
            return false;
        }

        true
    }

    fn validate_actor(
        &self,
        battle_actor: BattleActor,
        target: Coord,
        player: Player,
        is_attacking: bool,
    ) -> bool {
        let (piece, end_coord, is_moving) = match battle_actor {
            BattleActor::Static { coord } => {
                let Some(tile) = self.board.get(coord) else {
                    // TODO: Error handling
                    return false;
                };

                let Some(piece) = tile.piece_option else {
                    // TODO: Error handling
                    return false;
                };

                if piece.exhausted {
                    // TODO: Error handling
                    return false;
                }

                if piece.owner() != player {
                    // TODO: Error handling
                    return false;
                }

                (piece, coord, false)
            }
            BattleActor::Moving { from, to } => {
                if !self.can_move(from, to, player) {
                    // TODO: Error handling
                    return false;
                }

                (
                    self.board[from]
                        .piece_option
                        .expect("Already know it exists"),
                    to,
                    true,
                )
            }
        };

        piece.range().contains(&end_coord.distance(target))
            && piece.can_support(is_attacking, is_moving)
    }

    pub fn do_end_turn(&mut self) {
        // Resupply phase
        // TODO

        // Turn change
        self.current_player = -self.current_player;

        // Income phase
        // TODO
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
