# Rust ライブラリ

**コピーして使う**ことを優先します。Cargoは保守・テスト用で、提出コードの実行には必須ではありません。

## 目的別の入口

| 何をしたい？ | 開くファイル | 覚えておくこと |
| --- | --- | --- |
| 入力をまとめて読む | [ABCテンプレート](templates/abc.rs) | 1ファイルで動く、外部クレート不要 |
| 正の整数の各桁の和 | [digit_sum.rs](src/math/digit_sum.rs) | O(桁数)、0 は 0 |
| GCD / LCM | [gcd_lcm.rs](src/math/gcd_lcm.rs) | LCMはオーバーフロー対策で `Option<u64>` |
| ソート済み配列の挿入位置 | [binary_search.rs](src/search/binary_search.rs) | lower: 以上、upper: より大きい |
| 区間の合計を何度も求める | [prefix_sum.rs](src/array/prefix_sum.rs) | `[l, r)` の半開区間 |

## 実際の入力・スニペット

- [proconioテンプレート](templates/proconio.rs) — `cargo-compete` で `proconio` を使うとき（Cargoの依存関係が必要）
- [使用中のRustスニペット](snippets/rust.json) — `proconio`、`nibun`、`ketawa` など、20項目
- [元データからの修正内容](../docs/スニペット.md) — 整理内容と展開上の注意点
- [開発環境](../docs/開発環境と使い方.md) — `cargo-compete` と `ac-test` の連携
- [外部資料](../docs/外部資料.md) — Rust標準APIや競プロ解説を必要に応じて参照

**二分探索:** 単に境界の位置が欲しいなら `a.partition_point(|&v| v < x)` でもよい。自分の実装と標準APIの両方を残す。

## 動作確認

```sh
cd rust
cargo test
cargo fmt --check
```

`rust/src/` のコードが正本です。コードをコピーするときは、対応する関数と必要な `use` のみ取り込んでください。

## 新しいアルゴリズムの追加基準

一つのファイルに次の内容を揃えます。

- 使用する条件（ソート済み、非負など）
- 引数と戻り値・境界条件
- 計算量
- テスト（空、最小値、重複、境界など）

難しいものまで先に揃える必要はありません。実際に欲しくなった順に追加します。
