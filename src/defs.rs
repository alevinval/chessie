pub(crate) type Sq = u8;

pub(crate) type BitBoard = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Castling {
    None,
    Some { left: bool, right: bool },
}

impl Default for Castling {
    fn default() -> Self {
        Self::Some { left: true, right: true }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn castling_default() {
        assert_eq!(Castling::Some { left: true, right: true }, Castling::default());
    }
}
