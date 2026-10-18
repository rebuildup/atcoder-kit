# Rust

**使いたい処理を分野から探す**ための入口。

## 提出用テンプレート

**[署名付きテンプレート `template.rs`](template.rs)** — AtCoderの提出時にコピーして使う。リポジトリURL・固定コミットSHA・コードの出典URLを自動挿入済み。

テンプレートの中身は今使っている `read_i()` / `read_vi()` / `read_s()` と空の `main()`。**Scannerなどの仕組みは標準搭載しない**。

原本は [`templates/contest.rs`](templates/contest.rs)。`main` 更新時にActionsが `template.rs` を再生成する。自作アルゴリズム等を別ファイルからコピーした場合は、その出典URLも別途追記する。


## 基礎

- [入出力](基礎/入出力.md) — `read_i()`・`read_vi()`・`read_s()`、バイト列、複数値、出力
- [スニペット](スニペット.md) — エディタで定型文を短く展開する

## アルゴリズム

[分野別の一覧](アルゴリズム/README.md)

| 分野 | トピック | 主な内容 |
| --- | --- | --- |
| 探索 | [二分探索](アルゴリズム/探索/二分探索.md) | 最短・説明・改造・標準API |
| 配列 | [累積和](アルゴリズム/配列/累積和.md) | 区間和・条件の個数・文字列での応用 |

## 実践

- [文字列の区間クエリ](実践/文字列の区間クエリ.md) — 実際の解答を基に改善する

## 補助資料・コード

- [使用中のスニペットJSON](snippets/rust.json) / [元データ](snippets/rust.original.json)
- [行単位入力](templates/line_input.rs) / [proconio](templates/proconio.rs) / [Scanner](templates/abc.rs)
- [テスト可能な参考実装](src/lib.rs) — `rust/src/` にある関数集。通常の書き方として優先するものではない
- [AtCoderの環境・ツール](../atcoder/環境とツール.md)

知識の基本単位はトピックのMarkdown。`.rs` は再利用やテストの意味がある場合に使う。

[参考資料](../参考資料.md) / [学習状況](../学習状況.md)
