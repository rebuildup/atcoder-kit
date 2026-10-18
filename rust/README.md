# Rust：短く書く・理解する・改造する

`.rs` の一覧ではなく、**問題を解くとき「これをやりたい」から探せる目次**。基本はMarkdownから実際の使い方を確認する。

## まず使う書き方

| 何をしたいか | 参照先 | 主に何が載っているか |
| --- | --- | --- |
| 整数・配列・文字列の入力 | [入力と出力](notes/入力と出力.md) | `read_i` / `read_vi` / `read_s`、`into_bytes`、2変数へ代入 |
| Yes / No・答えの出力 | [入力と出力](notes/入力と出力.md#yes--no--答えの出力) | 直接の`println!`、関数、`ANS` / `YES` / `NO` / `YN` スニペット |
| 二分探索 | [二分探索](notes/二分探索.md) | 標準APIで最短、自前で短く、説明・改造、関数 |
| 累積和・一致回数 | [累積和](notes/累積和.md) | 短いループ、境界、文字列の出現回数 |
| 実際の問題の書き方 | [文字列の区間クエリ](notes/実戦例_文字列区間.md) | 既存コードを基準に、無駄な変数や出力の重複を減らす |

## 環境・テンプレート

- [Rustスニペット（エディタで展開）](snippets/rust.json)：実戦用の短い定型文。
- [元のスニペット20項目](snippets/rust.original.json)：原データ保管。
- [1行入力の例](templates/line_input.rs)：元の`read_*` を利用した例。
- [proconioの例](templates/proconio.rs)：依存クレートが利用できるとき。
- [Scannerの例](templates/abc.rs)：入力が複数行に分かれるなど、必要な場合だけ。**通常の推奨形ではない**。

## 関数として保守・テストしたい場合だけ使う`.rs`

| 内容 | `.rs` 実装 |
| --- | --- |
| 二分探索の関数 | [binary_search.rs](src/search/binary_search.rs) |
| 累積和の関数 | [prefix_sum.rs](src/array/prefix_sum.rs) |
| 各桁の和 | [digit_sum.rs](src/math/digit_sum.rs) |
| GCD / LCM | [gcd_lcm.rs](src/math/gcd_lcm.rs) |

これらは**Markdownの書き方より優先しない**。提出時は必要なコードだけコピーするか、Rust標準APIを使う。

テストを実行する場合は `cd rust && cargo test && cargo fmt --check`。実戦のコードをすべてこのライブラリの関数として再実装する必要はない。

[外部資料の索引](../docs/外部資料.md) / [学習状況](../docs/学習状況.md)
