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

## 条件に一致する個数（C++ の count / count_if）

配列の中で特定の値や条件を満たす要素数は `filter().count()` で求める。

```rust
let a = [2, 3, 2, 5, 4, 2, 6];
let equal_2 = a.iter().filter(|&&x| x == 2).count(); // 3
let ge_4 = a.iter().filter(|&&x| x >= 4).count();   // 3
let even_ge_4 = a.iter()
    .filter(|&&x| x >= 4 && x % 2 == 0)
    .count(); // 2
```

- `iter()` は `&i32` を順番に取り出し、`filter()` の条件にはそれへの参照が渡るため、`|&&x|` で整数 `x` として受け取る（`|x| **x == 2` でもよい）。
- `filter()` は元の配列を変更せず、条件を満たす要素のみを通す。`count()` はその個数を数える。走査は `O(n)`、追加メモリは `O(1)`。
- 配列全体の要素数なら `a.len()` を使う。特定の値の**出現回数を何種類も**求める場合は[HashMapによる集計](Map.md#出現回数を数えるhashmap)も選択肢。
- 区間ごとに条件を満たす個数を何度も問われるなら[累積和](../アルゴリズム/配列/累積和.md#2-値ではなく条件を満たす回数を数える)を検討する。

## 条件に合う要素・位置を探す（C++ の find / find_if）

配列を左から検索し、**値が欲しいなら `find()`、添字が欲しいなら `position()`** を使う。

```rust
let a = [3, 9, 4, 9, 6];

let value = a.iter().find(|&&x| x > 5).copied(); // Some(9)
let first = a.iter().position(|&x| x == 9);      // Some(1)
let last = a.iter().rposition(|&x| x == 9);      // Some(3)
let exists = a.contains(&7);                    // false
let any_even = a.iter().any(|&x| x % 2 == 0);    // true
```

- `find()` は最初に条件を満たした**要素への参照**を `Option<&T>` で返す。`copied()` は整数など `Copy` な値を `Option<T>` にする。
- `position()` は条件を満たす**最初の添字**、`rposition()` は**最後の添字**を `Option<usize>` で返す。存在しなければ `None`。上の `&&x` と `&x` の違いは、`find()` が要素への参照を条件に渡すため。
- 存在するかだけなら、値の一致は `contains(&値)`、条件判定は `any(条件)` が短い。
- C++の `find` / `find_if` はイテレータを返す。Rustでは**値と位置のどちらが必要か**でAPIを選ぶ。`unwrap()` は見つかると保証できる場合のみ使う。探索は最悪 `O(n)`、ソート不要。

### 全件・存在・不在を判定する（C++ の all_of / any_of / none_of）

```rust
let a = [2, 4, 7, 8];
let all_even = a.iter().all(|&x| x % 2 == 0); // false
let any_ge_7 = a.iter().any(|&x| x >= 7);     // true
let none_negative = !a.iter().any(|&x| x < 0); // true

let b = [0, 1, 4];
let ok = b.iter().all(|&x| x >= 0 && x < 5);  // true
```

- `all()` は全要素が条件を満たすか、`any()` は1要素でも満たすかを判定する。Rustのイテレータには `none()` がないので、`!any()` で「どれも満たさない」を表す。
- `all()` は反例が見つかると、`any()` は一致が見つかると探索を終える（最悪 `O(n)`）。空配列なら `all()` は `true`、`any()` は `false`。

## 最小値・最大値とその位置（C++ の min_element / max_element）

`iter().min()` / `iter().max()` は値への参照を `Option<&T>` で返す。空配列なら `None` なので、`unwrap()` は空でないと分かっている場合だけ使う。

```rust
let a = [6, 2, 9, 2, 9];
let lo = *a.iter().min().unwrap(); // 2
let hi = *a.iter().max().unwrap(); // 9

let first_lo = a.iter().position(|&x| x == lo).unwrap(); // 1
let last_hi = a.iter().enumerate()
    .max_by_key(|&(_, &x)| x).unwrap().0; // 4
```

- `position()` は条件に合う**最初の位置**を返す（見つからなければ `None`）。
- `enumerate()` は `(添字, 要素)` を作る。`min_by_key()` は同じ最小値なら**最初**、`max_by_key()` は同じ最大値なら**最後**の位置を選ぶ。
- C++の `min_element` / `max_element` はどちらも最初の一致位置を返す。Rustで最大値の**最初**の位置が必要なら `a.iter().position(|&x| x == hi)` を使う。
- いずれの探索も `O(n)`。整数などコピーできる値なら `a.iter().min().copied()` で `Option<T>` を返せる。

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
