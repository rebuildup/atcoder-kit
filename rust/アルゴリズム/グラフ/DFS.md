# DFS（深さ優先探索）

グラフの頂点を深くたどり、行き止まりで戻る探索方法。訪問済みの頂点は再訪問しない。

## 隣接リスト

```rust
let g = vec![
    vec![1, 3], // 頂点0から1と3
    vec![0, 2], // 頂点1から0と2
    vec![1],    // 頂点2から1
    vec![0],    // 頂点3から0
];
```

`g[v]` は「頂点 `v` から直接移動できる頂点の一覧」。`g[0]` なら `[1, 3]` なので、0から1と3へ移動できる。無向グラフの辺は両方向に登録する（`g[u].push(v)` と `g[v].push(u)`）。

## 再帰によるDFS

現在の頂点を訪問済みにし、未訪問の隣接頂点へ再帰的に進む。

```rust
let mut seen = vec![false; g.len()];
```

```rust
fn dfs(v: usize, g: &[Vec<usize>], seen: &mut [bool], order: &mut Vec<usize>) {
    seen[v] = true;
    order.push(v);
    for &to in &g[v] {
        if !seen[to] {
            dfs(to, g, seen, order);
        }
    }
}
```

### 呼び出し例

```rust
let mut seen = vec![false; g.len()];
let mut order = Vec::new();
dfs(0, &g, &mut seen, &mut order);
// order = [0, 1, 2, 3]
```

0→1→2と深く進み、2から戻った後に0→3へ進む。訪問順は隣接リストに書いた順番で変わる。`seen[v] = true`を再帰の前に行うのは、同じ辺を行き来しないため。

## 到達判定

DFSを実行した後、`seen[target]` が `true` なら始点から目標に到達できる。

## 連結成分数（無向グラフ）

未訪問の頂点からDFSを開始するたび、連結成分を1つ見つけたことになる。

```rust
let mut seen = vec![false; g.len()];
let mut order = Vec::new();
let mut components = 0;
for v in 0..g.len() {
    if !seen[v] {
        components += 1;
        dfs(v, &g, &mut seen, &mut order);
    }
}
// components = 1
```
