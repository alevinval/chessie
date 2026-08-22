pub type Sq = u8;

pub type BitBoard = u64;

// CastlingRights of the player (left, right)
pub(crate) type CastlingRights = (bool, bool);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CastlingUpdate {
    Left,
    Right,
    Both,
}
