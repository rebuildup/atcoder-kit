# map：HashMap と BTreeMap

キーと値の組を保存する連想配列。C++ の `unordered_map` に相当するのが `HashMap`、`map` に相当するのが `BTreeMap`。

| | `HashMap` | `BTreeMap` |
| --- | --- | --- |
| キーの走査順 | 保証なし | 昇順 |
| 検索・挿入・削除 | 平均 `O(1)` | `O(log n)` |
| 用途 | 出現回数などの集計 | キー順の列挙・範囲検索 |

## 出現回数を数える（HashMap）

```rust
use std::collections::HashMap;

fn main() {
    let mut count = HashMap::new();

    for x in [3, 1, 3, 2, 3, 1] {
        *count.entry(x).or_insert(0) += 1;
    }

    println!("{:?}", count.get(&3)); // Some(3)
}
```

`entry(x).or_insert(0)` は、キーが**なければ** `0` を挿入し、**あれば**既存の値をそのまま使う。戻り値は `&mut V` なので `*` で参照先を加算する。 `HashMap` の走査順には依存しない。

## 追加・上書きと昇順の走査（BTreeMap）

```rust
use std::collections::BTreeMap;

fn main() {
    let mut m = BTreeMap::new();

    m.insert(1, 10);
    m.insert(2, 20);
    *m.entry(1).or_insert(5) += 2; // 10 + 2 = 12
    *m.entry(3).or_insert(5) += 2; // 5 + 2 = 7
    m.insert(2, 30);               // 20 を 30 で上書き

    println!("{:?}", m.into_iter().collect::<Vec<_>>());
    // [(1, 12), (2, 30), (3, 7)]
}
```

`BTreeMap` の `into_iter()` は所有権を消費し、キーと値のタプル `(K, V)` を昇順に列挙する。タプルの表示は `(キー, 値)` で、`Some` は付かない。

## 共通の基本操作

| 操作 | 意味・戻り値 |
| --- | --- |
| `m.insert(k, v)` | 追加または上書き。以前の値を `Option<V>` で返す |
| `m.get(&k)` | 値の参照を `Option<&V>` で返す（例：`Some(12)`） |
| `m.contains_key(&k)` | キーが存在すれば `true` |
| `m.remove(&k)` | 削除した値を `Option<V>` で返す |
| `m.len()` | 異なるキーの数 |

存在しないキーの `get()` は `None`。数値などの値を取り出したいときは `m.get(&k).copied().unwrap_or(0)` と書ける（`V: Copy`）。標準APIで十分なため独自スニペットは追加しない。

## 参考

- [Rust公式：HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
- [Rust公式：BTreeMap](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html)
