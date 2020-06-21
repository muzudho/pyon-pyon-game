//! チェスは UCI、将棋は USI、ぴょんぴょんゲームは UXI☆（＾～＾）
use crate::board::{Board, Piece};

impl Board {
    /// TODO uxi を board に変換したいぜ☆（＾～＾）
    pub fn from_uxi(uxi: &str) -> Board {
        let mut board = Board::default();

        let mut i = 11;
        for ch in uxi.chars() {
            match ch {
                'x' => {
                    board.pieces[i] = Some(Piece::First);
                    i += 1;
                }
                'o' => {
                    board.pieces[i] = Some(Piece::Second);
                    i += 1;
                }
                '1' => {
                    board.pieces[i] = None;
                    i += 1;
                }
                '2' => {
                    board.pieces[i] = None;
                    i += 1;
                    board.pieces[i] = None;
                    i += 1;
                }
                '3' => {
                    board.pieces[i] = None;
                    i += 1;
                    board.pieces[i] = None;
                    i += 1;
                    board.pieces[i] = None;
                    i += 1;
                }
                '4' => {
                    for _ii in 0..4 {
                        board.pieces[i] = None;
                        i += 1;
                    }
                }
                '5' => {
                    for _ii in 0..5 {
                        board.pieces[i] = None;
                        i += 1;
                    }
                }
                '/' => {
                    i += 5;
                }
                ' ' => {
                    break;
                }
                _ => panic!("UXI error: {}", ch),
            }
        }

        board
    }
}
