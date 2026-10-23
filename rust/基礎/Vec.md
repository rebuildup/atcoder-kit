# Vec：可変長配列

C++ の `std::vector<T>` に相当する `Vec<T>`。要素を追加・削除でき、添字で参照できる。**`use` や外部クレートは不要**。

## 基本操作

そのまま `main()` として実行できる例：

```rust
fn main() {
    let mut v = vec![3, 5, 7];
    v.push(9);
    v[1] = 10;

    let x = v.pop().unwrap();
    let mut sum = 0;
    for &n in &v {
        sum += n;
    }

    println!("{} {} {}", v.len(), x, sum); // 3 9 20
}
```

- `vec![3, 5, 7]`：初期値を指定して作成。`v[0]` が先頭。
- `push(x)`：末尾に追加。`pop()`：末尾を取り出し、`Option<T>` を返す。空なら `None` なので `unwrap()` はパニックする。
- `let mut`：追加・削除・代入などで値を変更する場合に必要。
- `for &n in &v`：要素を借用して走査し、整数など `Copy` な値を取り出す。配列は消費しない。

## よく使う書き方

```rust
let mut v: Vec<i64> = Vec::new(); // 空
let a = vec![10, 20, 30];        // 初期値あり
let n = 5;
let zeros = vec![0_i64; n];       // 長さ n、すべて 0

v.push(10);
v.push(20);
let last = v.pop();  // Some(20)
let first = v.get(0); // Some(&10)、範囲外なら None
let count = v.len(); // 1
let empty = v.is_empty(); // false
v.clear();
```

| C++ | Rust |
| --- | --- |
| `vector<int> v;` | `let mut v = Vec::<i32>::new();` |
| `vector<int> v(n, 0);` | `let v = vec![0; n];` |
| `v.push_back(x)` | `v.push(x)` |
| `v.pop_back()` | `v.pop()` |
| `v.size()` | `v.len()` |
| `v[i]` | `v[i]` |
| `v.clear()` | `v.clear()` |

`v[i]` は範囲外でパニックする。範囲外を判定したい場合は `v.get(i)` を使う。末尾への `push` は償却 `O(1)`、`pop` と添字アクセスは `O(1)`。

入力から `n` 個の要素を作るときは [入出力](入出力.md) の `(0..n).map(|_| read()).collect()` を参照する。短い標準操作なので独自スニペットは追加しない。

## 参考

- [Rust公式：Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html)
