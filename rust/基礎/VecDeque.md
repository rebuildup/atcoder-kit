# VecDeque：キュー（先入れ先出し）

Rustのキューには標準ライブラリの `VecDeque<T>` を使う。先に入れた要素を先に取り出す（FIFO）。専用クレートは不要。

## 基本操作

```rust
use std::collections::VecDeque;

fn main() {
    let mut q = VecDeque::from([5, 7, 9]);

    let a = *q.front().unwrap(); // 5：参照のみ
    q.push_back(a + 1);          // [5, 7, 9, 6]
    let b = q.pop_front().unwrap(); // 5：先頭を削除
    let c = q.pop_front().unwrap(); // 7：先頭を削除

    println!("{} {} {} {:?}", a, b, c, q); // 5 5 7 [9, 6]
}
```

- `push_back(x)`：末尾へ追加（償却 `O(1)`）。
- `pop_front()`：先頭を取り出して削除（`O(1)`）、空なら `None`。
- `front()` / `back()`：先頭 / 末尾を削除せず参照し、`Option<&T>` を返す。
- `len()` / `is_empty()`：要素数 / 空かどうか。

`unwrap()` は `None` ならパニックする。要素がなくなるまで処理する場合は次の形が使える。

```rust
while let Some(x) = q.pop_front() {
    println!("{x}");
}
```

## C++との対応

| C++ `std::queue` | Rust `VecDeque` |
| --- | --- |
| `q.push(x)` | `q.push_back(x)` |
| `q.pop()` | `q.pop_front()` |
| `q.front()` / `q.back()` | `q.front()` / `q.back()` |
| `q.size()` / `q.empty()` | `q.len()` / `q.is_empty()` |

C++の `queue::pop()` は値を返さないが、Rustの `pop_front()` は値を返す。`Vec` の `pop()` と異なり、**取り出す側は先頭**。

キューはBFS（幅優先探索）などで使う。両端操作が必要なら、`VecDeque` は `push_front()` / `pop_back()` も使える。

## 参考

- [Rust公式：VecDeque](https://doc.rust-lang.org/std/collections/struct.VecDeque.html)
