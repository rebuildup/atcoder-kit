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

## 合計と畳み込み（C++ の accumulate）

配列全体の合計は `sum()`、初期値や計算方法を指定するなら `fold()` を使う。どちらも `use` は不要。

```rust
let a = [2_i64, 3, 4];

let total: i64 = a.iter().sum();                 // 9
let with_initial = a.iter().fold(10, |acc, &x| acc + x); // 19
let product = a.iter().fold(1, |acc, &x| acc * x);       // 24
```

- `sum()` は数値の合計。結果の型は `let total: i64` または `sum::<i64>()` で指定できる。
- `fold(初期値, |acc, &x| 計算式)` は、各要素を処理した結果を次の `acc` に渡す。上の `&x` は `iter()` が返す参照から整数を取り出す書き方。
- 積だけなら `a.iter().product::<i64>()` でもよい。いずれも `O(n)`。
- `sum()` は全体の値を1つ返す。[累積和](../アルゴリズム/配列/累積和.md)は各位置までの合計を保存し、区間和を繰り返し調べるときに使う。

## 反転と逆順の走査

```rust
let mut v = vec![1, 3, 5];

let a: Vec<_> = v.iter().rev().copied().collect();
// a = [5, 3, 1]、v = [1, 3, 5]

v.reverse();
// v = [5, 3, 1]
```

- `v.reverse()`：元の `Vec` を反転する（`O(n)`、追加メモリ `O(1)`）。
- `v.iter().rev()`：元の `Vec` を変更せず逆順に走査する。
- `copied().collect()`：値をコピーして別の `Vec` に保存する（`T: Copy` の場合）。元の `v` を後から変更しても `a` には影響しない。

文字列の反転は `String` に直接 `reverse()` できないため、例えば `s.chars().rev().collect::<String>()` を使う（結合文字の表示順には注意）。

## スタック（後入れ先出し）

Rustでは専用の `Stack` 型ではなく、`Vec<T>` を `push()` と `pop()` で使う。後から入れた値を先に取り出す（LIFO）。

```rust
let mut st = vec![2, 4, 6];

let a = *st.last().unwrap(); // 6、削除しない
st.push(a * 2);              // [2, 4, 6, 12]
let b = st.pop().unwrap();    // 12、末尾を削除
let c = st.pop().unwrap();    // 6、末尾を削除
// st = [2, 4]
```

- `push(x)`：末尾に追加（償却 `O(1)`）。
- `pop()`：末尾を削除して `Option<T>` を返す（`O(1)`）。空なら `None`。
- `last()`：削除せずに末尾を参照し `Option<&T>` を返す（`O(1)`）。例の `*` は整数の参照から値を取り出すため。
- `len()` / `is_empty()`：要素数 / 空かどうか。

C++ の `stack::top()` は Rust の `Vec::last()` に対応する。C++ の `stack::pop()` は値を返さないが、Rust の `Vec::pop()` は値を返す。

## 参考

- [Rust公式：Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html)
