// cargo-compete の問題用 Cargo.toml に proconio が入っている前提。
// 外部クレートを使える環境専用（AtCoder以外のCargoでも利用可能）。
// このファイル単体を rustc で直接コンパイルしても proconio は解決されない。
// 参照URL（生成AI製ライブラリコードを貼り付けた場合など）:
// -

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
    }

    // Usize1により頂点番号は0-indexedで得られる。
    // 問題に応じて入力の構造を書き換える。
    let _ = (n, edges);
}
