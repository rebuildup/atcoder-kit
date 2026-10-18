// AtCoder 用テンプレート（Rust、外部クレートなし）
// 著者: rebuildup
//
// ライブラリの関数をコピーした場合は、コンテスト前に公開済みの
// 固定コミットURLを下の欄へ追加する。未使用なら参照欄を削除する。
// 参照URL:
// - 

use std::io::{self, Read};

struct Scanner {
    input: Vec<u8>,
    index: usize,
}

impl Scanner {
    fn new() -> Self {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        Self {
            input: input.into_bytes(),
            index: 0,
        }
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        while self.index < self.input.len() && self.input[self.index].is_ascii_whitespace() {
            self.index += 1;
        }
        let start = self.index;
        while self.index < self.input.len() && !self.input[self.index].is_ascii_whitespace() {
            self.index += 1;
        }
        std::str::from_utf8(&self.input[start..self.index])
            .unwrap()
            .parse::<T>()
            .ok()
            .expect("入力を読み取れませんでした")
    }
}

fn main() {
    let mut sc = Scanner::new();

    // ここから問題ごとの処理を記述する。
    // 例: let n: usize = sc.next();
    // 例: let a: Vec<i64> = (0..n).map(|_| sc.next()).collect();

    // 未使用時の警告を抑える。入力を読む場合は消してよい。
    let _ = &mut sc;
}
