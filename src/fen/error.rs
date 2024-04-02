use core::fmt;

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum FenError {
    NotSupported(String),
    Invalid,
}

impl FenError {
    fn description(&self) -> String {
        match self {
            FenError::NotSupported(thing) => format!("{thing} not supported yet"),
            FenError::Invalid => "Invalid FEN".to_string(),
        }
    }
}

impl fmt::Display for FenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.description())
    }
}
