# HashSet：重複を除く・種類数を数える

**同じ値を何度入れても、集合には1個だけ残る。** 重複を無視して数えたいときは `HashSet` が使える。

## n個の整数から、異なる値の個数を数える

**提出テンプレートの `read<T>()` を使う例。** 整数が同じ行に並んでいても、複数行に分かれていても使える。

```rust
use std::collections::HashSet;

fn main() {
    let n: usize = read();
    let mut d = HashSet::new();
    for _ in 0..n {
        let x: i64 = read();
        d.insert(x);
    }
    println!("{}", d.len());
}
```

入力例（5個あるが、異なる値は3種類）：

```text
5
3
1
3
2
1
```

出力：

```text
3
```

- `use std::collections::HashSet;`：標準ライブラリから使う。外部クレートは不要。
- `HashSet::new()`：空の集合を作る。`insert` した値から要素型が推論される。
- `insert(x)`：まだない値だけを追加する。既にあれば集合の中身は増えない。
- `len()`：重複を除いた要素数を返す（`usize`）。
- `for _ in 0..n`：`n` 回繰り返すだけなら `_` でよい。`n` は通常 `usize` で受け取る。

`read()` は[標準テンプレート](../template.rs)にある。上の例は `main()` 部分を差し替え、`use` を追加して使う。**別の `read_i()` は不要**。

## 基本操作をコピーする

以下は `main` の中にコピーする形。`use` を関数内に書いても動作する。

```rust
use std::collections::HashSet;

let mut d = HashSet::new();
d.insert(10);
d.insert(20);
d.insert(10); // 重複なので要素数は増えない

let exists = d.contains(&10); // true
let count = d.len();         // 2
d.remove(&10);               // 10を削除
```

`insert(x)` の戻り値は新しく追加できたら `true`、既にあれば `false`。`remove(&x)` も削除できたかを `bool` で返す。`contains(&x)` は存在判定。

**`HashSet` の要素に順番はない。** `d[0]` のような添字アクセスもできない。追加・検索・削除は平均 `O(1)`、`n` 個の挿入は平均 `O(n)` だが、ハッシュ衝突などで最悪の計算量は変わる。

## エディタで展開する（スニペット）

[スニペット定義](../snippets/rust.json)を VS Code / Cursor に登録して使う。

| 接頭辞 | 展開するコード | 必要なもの |
| --- | --- | --- |
| `hashset` または `hset` | `use` と `let mut d = HashSet::new();` | 後から `insert` などで要素型を決める |
| `hsetn`（旧方式） | `use`、初期化、`n` 回 `read_i()` して `len()` を出力 | **旧来の行入力を使うときのみ**。変数 `n` と関数 `read_i()`（`stdri`）が必要 |

**`hashset` と `hsetn` は別の書き方なので、一度に両方を展開しない。** `use` は関数内に置けるため、どちらも `main` の中で展開できる。

`hsetn` が展開する**旧式コード**（現在の標準 `read<T>()` を使う場合は上の `for` の例を使う）：

```rust
use std::collections::HashSet;
let mut d = HashSet::new();
for _ in 0..n {
    d.insert(read_i());
}
println!("{}", d.len());
```

## 別の書き方：mapとcollect

`HashSet` へ要素を集めるなら、次も同じ結果になる。

```rust
let d: HashSet<i64> = (0..n).map(|_| read()).collect();
println!("{}", d.len());
```

ただし `HashSet` の `use` とテンプレートの `read<T>()` が必要。**今回は `for` と `insert` の方が処理を読み取りやすいので、短くするためだけに置き換えなくてよい。**

## BTreeSet：順序付き集合

C++の `set` に相当する。要素は重複せず、**昇順で走査**できる。C++の `unordered_set` に相当する `HashSet` は順序を保証しない。

```rust
use std::collections::BTreeSet;

let mut s = BTreeSet::from([5, 2, 5, 3]);
s.insert(1);
s.remove(&3);
let a: Vec<_> = s.into_iter().collect(); // [1, 2, 5]
```

`insert`・`contains`・`remove`・`len` は `HashSet` と同様に使える。検索・追加・削除はいずれも `O(log n)`。

## 集合演算

```rust
let a = BTreeSet::from([1, 2, 3]);
let b = BTreeSet::from([2, 3, 4]);
let common: Vec<_> = a.intersection(&b).copied().collect(); // [2, 3]
```

- `intersection(&b)`：積集合（共通部分）
- `union(&b)`：和集合
- `difference(&b)`：差集合（`a` にだけある値）

`HashSet` でも同じ演算を使えるが、結果の走査順は保証されない。`BTreeSet` では昇順になる。

## 参考

- [Rust公式：BTreeSet](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html)
- [Rust公式：HashSet](https://doc.rust-lang.org/std/collections/struct.HashSet.html)
- [Rust by Example 日本語：HashSet](https://doc.rust-jp.rs/rust-by-example-ja/std/hash/hashset.html)
