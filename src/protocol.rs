//! チェスは UCI、将棋は USI、ぴょんぴょんゲームは UXI☆（＾～＾）
use crate::board::{Board, Piece};

impl Board {
    /// TODO xfen を board に変換したいぜ☆（＾～＾）
    pub fn from_xfen(xfen: &str) -> Option<Board> {
        if !xfen.starts_with("xfen ") {
            return None;
        }

        let mut board = Board::default();

        // 文字数☆（＾～＾）
        let mut count = -1isize;
        // 番地☆（＾～＾）
        let mut addr = 11;
        // Rust言語では文字列に配列のインデックスを使ったアクセスはできないので、
        // 一手間かけるぜ☆（＾～＾）
        for ch in xfen.chars() {
            // 先にカウントアップ☆（＾～＾）
            count += 1;
            if count < "xfen ".len() as isize {
                // 先頭のキーワードは読み飛ばすぜ☆（＾～＾）
                continue;
            }
            match ch {
                'x' => {
                    board.pieces[addr] = Some(Piece::First);
                    addr += 10;
                }
                'o' => {
                    board.pieces[addr] = Some(Piece::Second);
                    addr += 10;
                }
                '1' => addr += 10,
                '2' => addr += 20,
                '3' => addr += 30,
                '4' => addr += 40,
                '5' => addr += 50,
                '/' => addr = addr % 10 + 11,
                ' ' => break,
                _ => panic!("UXI error: {}", ch),
            }
        }

        Some(board)
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

        // 合法手チェック☆（＾～＾）
        // 移動先のマスに駒があってはダメ☆（＾～＾）
        if let Some(_piece_val) = self.pieces[dst_sq as usize] {
            panic!(
                "移動先のマスに駒があってはダメだぜ☆（＾～＾） dst_sq={}",
                dst_sq
            )
        }

        self.pieces[dst_sq as usize] = self.pieces[src_sq as usize];
        self.pieces[src_sq as usize] = None;
    }
}
