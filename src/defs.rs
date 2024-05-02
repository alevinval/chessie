pub(crate) type Sq = u8;

pub(crate) type BitBoard = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CastlingUpdate {
    None,
    Left,
    Right,
    Both,
}
