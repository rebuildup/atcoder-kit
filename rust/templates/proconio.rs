// cargo-compete の問題用 Cargo.toml に proconio が入っている前提。
// このファイル単体を rustc で直接コンパイルしても proconio は解決されない。
// 参照URL（生成AI製ライブラリコードを貼り付けた場合など）:
// -

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    // 入力内容と問題に応じて変更する。
    let _ = (n, a);
}
