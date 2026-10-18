# HashSet：重複を除く・種類数を数える

**同じ値を何度入れても、集合には1個だけ残る。** 重複を無視して数えたいときは `HashSet` が使える。

## n個の整数から、異なる値の個数を数える

そのままコピーして使える例。各整数が**1行に1個ずつ**与えられる場合を想定する。

```rust
use std::collections::HashSet;

fn read_i() -> i64 {
    let mut b = String::new();
    std::io::stdin().read_line(&mut b).unwrap();
    b.trim().parse::<i64>().unwrap()
}

fn main() {
    let n = read_i();
    let mut d = HashSet::new();
    for _ in 0..n {
        d.insert(read_i());
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
- `for _ in 0..n`：`n` 回繰り返すだけなら `_` でよい。今回は配列の添字がないので、`n` が `i64` でも問題ない。

注意：`read_i()` は1行から整数を1個読む。数値が同じ行に複数並ぶ入力では、入力方法を変更する。

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
| `hsetn` | `use`、初期化、`n` 回 `read_i()` して `len()` を出力 | 変数 `n`、関数 `read_i()`（`stdri` で用意できる） |

**`hashset` と `hsetn` は別の書き方なので、一度に両方を展開しない。** `use` は関数内に置けるため、どちらも `main` の中で展開できる。

`hsetn` が展開するコード（変数名や `n`、読み取り部分は編集できる）：

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
let d: HashSet<_> = (0..n).map(|_| read_i()).collect();
println!("{}", d.len());
```

ただし `HashSet` の `use` と `read_i()` の定義が必要。**今回は `for` と `insert` の方が処理を読み取りやすいので、短くするためだけに置き換えなくてよい。**

## 参考

- [Rust公式：HashSet](https://doc.rust-lang.org/std/collections/struct.HashSet.html)
- [Rust by Example 日本語：HashSet](https://doc.rust-jp.rs/rust-by-example-ja/std/hash/hashset.html)
