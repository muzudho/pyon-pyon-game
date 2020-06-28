use std::fmt;

pub struct CommandLine {
    line: String,
    len: usize,
    starts: usize,
}
impl CommandLine {
    pub fn new(line: &str) -> Self {
        // 末尾の改行を除こうぜ☆（＾～＾）
        // trim すると空白も消えるぜ☆（＾～＾）
        let line: String = match line.trim().parse() {
            Ok(n) => n,
            Err(e) => panic!(format!("(Err.38)  Failed to parse. / {}", e)),
        };
        // 文字数を調べようぜ☆（＾～＾）
        let len = line.chars().count();
        let starts = 0;
        CommandLine {
            line: line,
            len: len,
            starts: starts,
        }
    }

    pub fn starts_with(&self, expected: &str) -> bool {
        let len2 = expected.len();
        len2 <= self.len && &self.line[self.starts..len2] == expected
    }

    pub fn go_next_to(&mut self, expected: &str) {
        self.starts += expected.len();
    }

    pub fn rest(&self) -> &str {
        &self.line[self.starts..]
    }
}
impl fmt::Debug for CommandLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
        // 文字列を タテボウで クォートする(挟む)のは わたしの癖で、
        // |apple|banana|cherry| のように区切れる☆（＾～＾）
        // そのうち めんどくさくなったら お前もこうなる☆ｍ９（＾～＾）
        "line=|{}| len={} starts={}",
            self.line, self.len, self.starts
        )
    }
}
