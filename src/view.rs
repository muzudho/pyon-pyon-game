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
            "{}{}",
            format!(
                "[{0} move(s) | Go! ? | 0 repeat(s)]

",
                0
            ),
            format!(
                "   +---+---+---+---+---+
 1 |{0:^3}|{1:^3}|{2:^3}|{3:^3}|{4:^3}|
   +---+---+---+---+---+
 2 |{5:^3}|{6:^3}|{7:^3}|{8:^3}|{9:^3}|
   +---+---+---+---+---+
 3 |{10:^3}|{11:^3}|{12:^3}|{13:^3}|{14:^3}|
   +---+---+---+---+---+
 4 |{15:^3}|{16:^3}|{17:^3}|{18:^3}|{19:^3}|
   +---+---+---+---+---+
 5 |{20:^3}|{21:^3}|{22:^3}|{23:^3}|{24:^3}|
   +---+---+---+---+---+
     a   b   c   d   e
",
                p(self.pieces[11]),
                p(self.pieces[21]),
                p(self.pieces[31]),
                p(self.pieces[41]),
                p(self.pieces[51]),
                p(self.pieces[12]),
                p(self.pieces[22]),
                p(self.pieces[32]),
                p(self.pieces[42]),
                p(self.pieces[52]),
                p(self.pieces[13]),
                p(self.pieces[23]),
                p(self.pieces[33]),
                p(self.pieces[43]),
                p(self.pieces[53]),
                p(self.pieces[14]),
                p(self.pieces[24]),
                p(self.pieces[34]),
                p(self.pieces[44]),
                p(self.pieces[54]),
                p(self.pieces[15]),
                p(self.pieces[25]),
                p(self.pieces[35]),
                p(self.pieces[45]),
                p(self.pieces[55]),
            )
        );
    }
}
