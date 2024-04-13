#[cfg(test)]
pub(crate) use decode::decode;
pub(crate) use encode::encode;
pub(crate) use error::FenError;

mod decode;
mod encode;
mod error;
