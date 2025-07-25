pub mod coordinates;
pub mod piece;
pub mod tile;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Player {
    P1,
    P2
}