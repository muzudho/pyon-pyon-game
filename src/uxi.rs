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
                    i += 10;
                }
                'o' => {
                    board.pieces[i] = Some(Piece::Second);
                    i += 10;
                }
                '1' => i += 10,
                '2' => i += 20,
                '3' => i += 30,
                '4' => i += 40,
                '5' => i += 50,
                '/' => i = i % 10 + 11,
                ' ' => break,
                _ => panic!("UXI error: {}", ch),
            }
        }

        board
    }

    /// TODO 指す
    /// 最初は、合法か判定せずに　とりあえず動かせだぜ☆（＾～＾）
    ///
    /// # Arguments
    ///
    /// * `move_` - 指し手。`b5c3` など。
    pub fn do_(&mut self, move_: &str) {
        fn a_to_u8(ch: char) -> u8 {
            match ch {
                'a' => 1,
                'b' => 2,
                'c' => 3,
                'd' => 4,
                'e' => 5,
                _ => panic!("知らないアルファベットだぜ☆（＾～＾） ch={}", ch),
            }
        }
        fn n_to_u8(ch: char) -> u8 {
            match ch {
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                _ => panic!("知らない数字だぜ☆（＾～＾） ch={}", ch),
            }
        }

        let mut src_sq = 0;
        let mut dst_sq = 0;

        for (i, ch) in move_.chars().enumerate() {
            match i {
                0 => {
                    src_sq += 10 * a_to_u8(ch);
                }
                1 => {
                    src_sq += n_to_u8(ch);
                }
                2 => {
                    dst_sq += 10 * a_to_u8(ch);
                }
                3 => {
                    dst_sq += n_to_u8(ch);
                }
                _ => panic!("知らない構文だぜ☆（＾～＾） i={}", i),
            }
        }

        println!(
            "Debug   | do_ move_={} src_sq={} dst_sq={}",
            move_, src_sq, dst_sq
        );

        self.pieces[dst_sq as usize] = self.pieces[src_sq as usize];
        self.pieces[src_sq as usize] = None;
    }
}
