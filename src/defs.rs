pub type Sq = u8;
pub type BitBoard = u64;

pub enum Dir {
    Up(u8),
    Down(u8),
    Right(u8),
    Left(u8),
    Custom(i8, i8),
}
