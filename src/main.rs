mod board;
mod uxi;

use crate::board::{Board, Piece};
use std;

fn main() {
  println!(
    "ぴょんぴょんゲーム

コマンド:
`pos` - 局面表示。"
  );

  let board = Board::from_uxi("xo1o1/o1o1o/5/x1x1x/1x1x1 x");

  // [Ctrl]+[C] で強制終了
  loop {
    let (line, len, starts) = receipt_message();

    println!("Debug   | {} {} {}", line, len, starts);
    if 2 < len && &line[starts..3] == "pos" {
      pos(&board);
    }
  }
}

fn receipt_message() -> (String, usize, usize) {
  let mut line: String = String::new();

  // まず最初に、コマンドライン入力を待機しろだぜ☆（＾～＾）
  match std::io::stdin().read_line(&mut line) {
    Ok(_n) => {}
    Err(e) => panic!(format!("(Err.28)  Failed to read line. / {}", e)),
  };

  // 末尾の改行を除こうぜ☆（＾～＾）
  // trim すると空白も消えるぜ☆（＾～＾）
  let line: String = match line.trim().parse() {
    Ok(n) => n,
    Err(e) => panic!(format!("(Err.38)  Failed to parse. / {}", e)),
  };

  // 文字数を調べようぜ☆（＾～＾）
  let len = line.chars().count();
  let starts = 0;

  (line, len, starts)
}

fn pos(board: &Board) {
  fn p(piece: Option<Piece>) -> String {
    if let Some(piece_val) = piece {
      piece_val.to_string()
    } else {
      " ".to_string()
    }
  };

  // 局面表示
  println!(
    "\
[0 move(s) | Go! ? | 0 repeat(s)]

   +---+---+---+---+---+
 1 | {} | {} |   | o |   |
   +---+---+---+---+---+
 2 | o |   | o |   | o |
   +---+---+---+---+---+
 3 |   |   |   |   |   |
   +---+---+---+---+---+
 4 | x |   | x |   | x |
   +---+---+---+---+---+
 5 |   | x |   | x |   |
   +---+---+---+---+---+
     a   b   c   d   e
",
    p(board.pieces[11]),
    p(board.pieces[21])
  );
}
