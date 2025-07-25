use std::ops::{Index, IndexMut, Neg};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Player {
    P1,
    P2,
}

impl Neg for Player {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::P1 => Self::P2,
            Self::P2 => Self::P1,
        }
    }
}

impl<T> Index<Player> for [T; 2] {
    type Output = T;

    fn index(&self, index: Player) -> &Self::Output {
        match index {
            Player::P1 => &self[0],
            Player::P2 => &self[1],
        }
    }
}

impl<T> IndexMut<Player> for [T; 2] {
    fn index_mut(&mut self, index: Player) -> &mut Self::Output {
        match index {
            Player::P1 => &mut self[0],
            Player::P2 => &mut self[1],
        }
    }
}
