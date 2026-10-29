# Union-Find（DSU）：グループの結合と所属判定

**要素のグループを結合し、2要素が同じグループに属するか**を高速に調べる。無向グラフの辺を追加しながら連結判定をする場合などに使う。辺をたどって調べる[DFS](DFS.md)・[BFS](BFS.md)と異なり、追加された辺によるグループのつながりを保持する。

## Rust：経路圧縮 + Union by Size

```rust
struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn leader(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            return x;
        }
        let p = self.parent[x];
        let root = self.leader(p);
        self.parent[x] = root; // 経路圧縮
        root
    }

    fn unite(&mut self, a: usize, b: usize) {
        let mut x = self.leader(a);
        let mut y = self.leader(b);
        if x == y {
            return; // すでに同じグループ
        }
        if self.size[x] < self.size[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.parent[y] = x; // 小さいグループを大きい方につなぐ
        self.size[x] += self.size[y];
    }

    fn same(&mut self, a: usize, b: usize) -> bool {
        self.leader(a) == self.leader(b)
    }
}

fn main() {
    let mut dsu = Dsu::new(5);
    dsu.unite(0, 1);
    dsu.unite(2, 3);
    dsu.unite(1, 2);

    println!("{}", dsu.same(0, 3)); // true
    println!("{}", dsu.same(0, 4)); // false
}
```

## 操作と内部の意味

- `new(n)`：最初は`0..n`の各要素が独立したグループ。各要素は自分を親に持ち、サイズは1。
- `leader(x)`：`parent`をたどってグループの代表（根）を返す。**経路圧縮**で途中の要素も直接根につなぎ直すため、次の探索が速くなる。
- `unite(a, b)`：両者の根を探し、異なる場合だけ結合する。**Union by Size**で小さいグループの根を大きいグループの根につなぎ、木が深くなるのを抑える。`size[x]`は根`x`が代表するグループの要素数。
- `same(a, b)`：両者の代表が一致するか比較する。代表の**番号は結合で変わる**ので、特定の代表番号を固定して扱わない。
- この例では結合後のグループは`{0, 1, 2, 3}`と`{4}`。`unite`は同じグループ同士でも何もしない。

## 計算量・使い分け

- 要素数を`n`とすると、初期化は時間・メモリ`O(n)`。経路圧縮とUnion by Sizeを併用した`leader`・`unite`・`same`は**償却`O(α(n))`**（逆アッカーマン関数。実用上ほぼ定数時間）。
- **辺の追加と連結判定**には向くが、通常のDSUは**結合後の分離（辺の削除）**を直接扱えない。最短距離や有向グラフの到達判定も求められない。
- 引数の要素番号は`0..n`の範囲内であること。外部クレートは不要。
