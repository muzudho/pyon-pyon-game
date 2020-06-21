use std::fmt;

/// 動くものは、石ではなくて駒だぜ☆（＾～＾）
#[derive(Clone, Copy)]
pub enum Piece {
    // 先手☆（＾～＾）
    First,
    // 後手☆（＾～＾）
    Second,
}
impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Piece::First => write!(f, "x"),
            Piece::Second => write!(f, "o"),
        }
    }
}

/// 盤
pub struct Board {
    // 枠あり☆（＾～＾）
    // 使ってないスペースが多くなるが、10進数で番地指定したいんで、10x10にするぜ☆（＾～＾）
    pub pieces: [Option<Piece>; 10 * 10],
}
impl Default for Board {
    fn default() -> Self {
        Board {
            pieces: [None; 10 * 10],
        }
    }
}
