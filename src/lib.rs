pub mod coordinates;
pub mod piece;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Player {
    P1,
    P2
}