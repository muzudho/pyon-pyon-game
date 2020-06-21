use crate::board::{Board, Piece};

impl Board {
    pub fn pos(&self) {
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
            p(self.pieces[11]),
            p(self.pieces[21])
        );
    }
}
