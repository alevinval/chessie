pub type Sq = u8;

pub type BitBoard = u64;

pub(crate) type CastlingTuple = (bool, bool);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CastlingUpdate {
    Left,
    Right,
    Both,
}
