pub(crate) type Sq = u8;

pub(crate) type BitBoard = u64;

pub(crate) type CastlingTuple = (bool, bool);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CastlingUpdate {
    Left,
    Right,
    Both,
}
