# BinaryHeap：優先度付きキュー

標準ライブラリの `BinaryHeap<T>` は、追加順ではなく値の優先順位で要素を取り出す。**標準は最大値優先**。外部クレートは不要。

## 最大値から取り出す

```rust
use std::collections::BinaryHeap;

fn main() {
    let mut q = BinaryHeap::from([3, 8, 5]);
    let a = q.pop().unwrap();   // 8 を取り出して削除
    q.push(6);
    let b = *q.peek().unwrap(); // 6 を参照（削除しない）

    println!("{a} {b}"); // 8 6
}
```

`peek()` は `Option<&T>`、`pop()` は `Option<T>` を返す。空なら `None` なので、`unwrap()` の前には要素があることが必要。

## 最小値から取り出す

`Reverse<T>` で比較順を逆転させる。

```rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let mut q = BinaryHeap::from([Reverse(3), Reverse(8), Reverse(5)]);

    let a = q.pop().unwrap().0; // 3
    q.push(Reverse(2));
    let b = q.pop().unwrap().0; // 2

    println!("{a} {b}"); // 3 2
}
```

`.0` は `Reverse(x)` の内側の値を取り出す。ヒープ内へ入れる値は `Reverse(x)` で統一する。

## よく使う操作

| C++ `priority_queue` | Rust `BinaryHeap` |
| --- | --- |
| `push(x)` | `push(x)` |
| `top()` | `peek()` |
| `pop()` | `pop()` |
| `size()` | `len()` |
| `empty()` | `is_empty()` |

C++の `priority_queue::pop()` は値を返さないが、Rustの `BinaryHeap::pop()` は削除した値を返す。

| 操作 | 計算量 |
| --- | --- |
| `push()` | 最悪 `O(log n)` |
| `pop()` | `O(log n)` |
| `peek()` | `O(1)` |
| `len()` / `is_empty()` | `O(1)` |

`BinaryHeap` の内部格納順はソート順ではない。`pop()` の繰り返しで優先順に取り出せる。ダイクストラ法など、次に処理する最小・最大の要素を効率よく選びたい場面で使う。

## 参考

- [Rust公式：BinaryHeap](https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html)
- [Rust公式：Reverse](https://doc.rust-lang.org/std/cmp/struct.Reverse.html)
