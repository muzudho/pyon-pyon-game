mod board;
mod command_line;
mod protocol;
mod view;

use crate::board::Board;
use crate::command_line::CommandLine;
use std;

fn main() {
  println!(
    "ぴょんぴょんゲーム

コマンド:
`do b5c3` - b5 の駒を c3 へ移動。
`pos` - 局面表示。"
  );

  let xfen = "xfen 1o1o1/o1o1o/5/x1x1x/1x1x1 x";
  if let Some(mut board) = Board::from_xfen(xfen) {
    // [Ctrl]+[C] でループを終了
    loop {
      let mut line: String = String::new();
      // まず最初に、コマンドライン入力を待機しろだぜ☆（＾～＾）
      match std::io::stdin().read_line(&mut line) {
        Ok(_n) => {}
        Err(e) => panic!(format!("(Err.28)  Failed to read line. / {}", e)),
      };

      // コマンドライン☆（＾～＾） p は parser の意味で使ってるぜ☆（＾～＾）
      let mut p = CommandLine::new(&line);

      if p.starts_with("pos") {
        board.pos();
      } else if p.starts_with("do") {
        p.go_next_to("do ");
        println!("Debug   | rest=|{}|", p.rest());
        board.do_(p.rest());
      } else {
        println!("Debug   | Command not found. {:?}", p);
      }
    }
  } else {
    panic!(format!("(Err.31)  xfen fail. / {}", xfen))
  }
}
