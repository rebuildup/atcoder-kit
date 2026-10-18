# Tampermonkey スクリプト一覧

**目的:** 現在使用しているAtCoder向けブラウザ環境を、日本語で見直し・再構築できるように記録する。

- **記録日:** 2026-10-18
- **出典:** ユーザーが共有したTampermonkey管理画面のスクリーンショット
- **確認できる範囲:** 表示名・有効/無効のみ。インストール元URL、作者、バージョン、実際の設定内容・挙動は、この画面だけでは確定できない。
- **状態:** 有効 **13件** / 無効 **1件** / 計 **14件**

## インストール済み

**表示順・表記はスクリーンショットに従う。** 「用途の目安」は名前からの推測であり、実動作の検証結果ではない。

| スクリプト名（表示名） | 状態 | 用途の目安（未検証） |
| --- | --- | --- |
| `ac-predictor` | 有効 | パフォーマンスなどの予測機能か |
| `ac-predictor-minimal` | 有効 | 予測機能の簡易版か |
| `AtCoder Dropdown Tasks` | 有効 | 問題一覧のドロップダウン表示か |
| `AtCoder Easy Test v2` | **無効** | テスト補助か |
| `AtCoder Submission User Colorizer` | 有効 | 提出一覧のユーザー表示を色分けするものか |
| `AtCoder → cargo-compete testcase bridge` | 有効 | cargo-competeテストケースへの連携か |
| `atcoder-difficulty-display` | 有効 | 問題難易度の表示か |
| `atcoder_all_open` | 有効 | 複数リンクの一括オープンか |
| `AtCoderLanguageButtons` | 有効 | 言語表示切り替えのボタンか |
| `AtCoderPerformanceColorizer` | 有効 | パフォーマンス表示の色分けか |
| `AtCoderResultNotifier` | 有効 | 結果通知か |
| `AtCoderStandingsAnalysis` | 有効 | 順位表の分析か |
| `New AtCoder Perf Graph` | 有効 | パフォーマンス推移グラフか |
| `解説ぜんぶ見る` | 有効 | 解説の一覧表示補助か |

## 自作ツール `ac-test` との関係

[自作 `ac-test` リポジトリ](https://github.com/rebuildup/ac-test) には、[Tampermonkey向け `ac-test.user.js`](https://github.com/rebuildup/ac-test/blob/master/ac-test.user.js) が存在する。

そのファイルのUserscriptメタデータを確認した結果は次のとおり。

- `@name ac-test`
- `@namespace local.ac-test`
- `@version 0.2.1`（リポジトリ確認時点）
- `@match https://atcoder.jp/contests/*/tasks/*`
- ローカル `http://127.0.0.1:27121/testcases` にテストケースを送る仕組み。

**注意:** スクリーンショットにある表示名は `AtCoder → cargo-compete testcase bridge` で、ソースの `@name ac-test` と一致しない。**同じスクリプトなのか、改名・別実装なのかは未確認。** `ac-test` と同一だと決めつけず、Tampermonkeyの対象スクリプトを開いて `@name` / `@namespace` / 送信先を確認する。

[ac-test と cargo-compete の役割分担](開発環境と使い方.md#ac-test-と-cargo-compete-の役割分担) も参照。

## 次に確認すること

再セットアップで困らないよう、各スクリプトについて次を追記する。

1. **インストール元URL**（Greasy Fork / GitHub / 個人ブログ等）。未確認のURLは捏造せず空欄にする。
2. **作者・バージョンと更新方法**（自動更新の有無）。
3. **実際に必要な機能**と使う場面、重複や競合の有無（特に `ac-predictor` と `ac-predictor-minimal`）。
4. **スクリプト権限と接続先**（`@grant`, `@connect`, `@match`）。外部への送信や自動更新を伴うものはソースを確認。
5. 無効の `AtCoder Easy Test v2` は、再度使う理由があるか確認する。

**記録方法:** 管理画面のエディタやスクリプトの配布ページからメタデータを確認し、URLと用途を追記する。第三者のスクリプト本体は出典・ライセンスが分からない状態でこのリポジトリへ転載しない。外部スクリプトの利用可否はコンテストのその時点のルールに従う。

この一覧は画面上の導入状況を記録したものであり、14件の安全性・利用規約への適合・相互互換性を保証するものではない。
