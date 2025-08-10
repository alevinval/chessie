use core::fmt;

#[derive(Debug, Eq, PartialEq)]
pub enum FenError {
    NotSupportedEnPassant,
    NotSupportedHalfMove,
    Invalid,
}

impl FenError {
    fn description(&self) -> &str {
        match self {
            FenError::NotSupportedEnPassant => "Chessie does not support en-passant yet",
            FenError::NotSupportedHalfMove => "Chessie does not support fifty-move rule yet",
            FenError::Invalid => "Invalid FEN",
        }
    }
}

impl fmt::Display for FenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.description())
    }
}
