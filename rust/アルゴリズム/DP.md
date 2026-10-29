# 動的計画法（DP）：基本・ナップサック・区間・bitDP

**状態（dpの各要素が何を表すか）・初期値・遷移・更新順序**を決め、同じ部分問題を繰り返し解かずに結果を再利用する。問題によって計算量も状態も変わる。

## 1. 基本DP：1段か2段進む

`dp[i]` = 0段目からちょうど`i`段目へ到達する方法数。直前は`i-1`または`i-2`段目なので、その方法数を足す。

```rust
let n = 5;
let mut dp = vec![0usize; n + 1];
dp[0] = 1; // 何もせず0段目にいる方法は1通り

for i in 1..=n {
    dp[i] += dp[i - 1];
    if i >= 2 {
        dp[i] += dp[i - 2];
    }
}
println!("{}", dp[n]); // 8
```

時間`O(n)`、メモリ`O(n)`。加算結果が`usize`に収まることを前提とする。直前の2値だけを持てばメモリ`O(1)`にもできる。

## 2. 0/1ナップサックDP：各品物を選ぶか選ばないか

`dp[w]` = **重さの合計が`w`以下**となるよう選んだときの最大価値。各品物は1度しか使えない。

```rust
let items = [(2usize, 3i64), (3, 4), (4, 5)];
let capacity = 5;
let mut dp = vec![0i64; capacity + 1];

for &(weight, value) in &items {
    for w in (weight..=capacity).rev() {
        dp[w] = dp[w].max(dp[w - weight] + value);
    }
}
println!("{}", dp[capacity]); // 7（重さ2と3を選ぶ）
```

**品物ごとに容量を降順に更新する**のが重要。昇順だと、その品物を使った直後の値を再利用して同じ品物を複数回選んだ計算になることがある。品物数`n`、容量`W`で時間`O(nW)`、メモリ`O(W)`。容量が極端に大きい場合はこの方法が適さないこともある。

## 3. 区間DP：隣り合うファイルを結合する

サイズ`a[l..r]`のファイルをすべて1つにする最小コストを`dp[l][r]`に保存する（**半開区間`[l, r)`**）。隣り合う2つのまとまりしか結合できず、結合コストは結合後のサイズ。

最後の結合位置を`k`として、`dp[l][k] + dp[k][r] + sum(l, r)`を最小化する。区間合計`sum(l, r)`には既存の[累積和](配列/累積和.md)を使う。

```rust
let a = [1_i64, 2, 3];
let n = a.len();
let mut p = vec![0i64; n + 1];
for i in 0..n {
    p[i + 1] = p[i] + a[i];
}

let mut dp = vec![vec![0i64; n + 1]; n + 1];
for len in 2..=n {
    for l in 0..=n - len {
        let r = l + len;
        dp[l][r] = i64::MAX;
        for k in l + 1..r {
            let cost = dp[l][k] + dp[k][r] + (p[r] - p[l]);
            dp[l][r] = dp[l][r].min(cost);
        }
    }
}
println!("{}", dp[0][n]); // 9（先に1と2を結合）
```

区間長`len`を短い順に処理することで、必要な小区間の結果が先に決まる。時間`O(n³)`、メモリ`O(n²)`。この例は`n >= 1`、サイズ・コストが非負で`i64`に収まることを想定する。

## 4. bitDP：訪問済みの場所と現在地

`dp[mask][v]` = **訪問済みの集合が`mask`で、現在地が`v`のときの最小移動コスト**。場所0から出発し、全場所を1回ずつ訪問する。出発点には戻らない。

```rust
let cost = [
    [0_i64, 2, 5],
    [2, 0, 1],
    [5, 4, 0],
];
let n = cost.len();
let inf = i64::MAX / 4;
let full = 1usize << n;
let mut dp = vec![vec![inf; n]; full];
dp[1][0] = 0; // 場所0のみ訪問済み

for mask in 0..full {
    for v in 0..n {
        if dp[mask][v] == inf {
            continue;
        }
        for to in 0..n {
            if mask & (1usize << to) != 0 {
                continue; // 訪問済み
            }
            let next = mask | (1usize << to);
            let candidate = dp[mask][v] + cost[v][to];
            dp[next][to] = dp[next][to].min(candidate);
        }
    }
}
let ans = dp[full - 1].iter().min().unwrap();
println!("{ans}"); // 3（0 → 1 → 2）
```

`mask`による集合の意味は[ビット操作](../基礎/ビット操作.md)、全部分集合との違いは[bit全探索](全探索.md#選ぶ選ばないbit全探索)を参照。bitDPは**同じ集合・同じ現在地の最小コストだけを保存**する。

時間`O(n² × 2^n)`、メモリ`O(n × 2^n)`。状態数が急増するため`n`が小さい場合に使う。ビットシフトの幅より`n`が小さいことと、コストの加算が`i64`に収まることを前提とする。経路が成立しない場合は`inf`が残るので、出力前に判定する。

## 選び方

| 状態の例 | 使う場面 |
| --- | --- |
| `dp[i]`：位置・手数までの結果 | 階段・数列・最小コスト |
| `dp[w]`：容量までの最大価値 | 各品物を0回か1回選ぶ |
| `dp[l][r]`：区間の最適値 | 隣接区間の結合・分割 |
| `dp[mask][v]`：選択済み集合と現在地 | 巡回・順序決定 |

**記録済みの理解は確認問題への正答であり、実問題での解法選択や自力実装を保証しない。**
