// 出典リポジトリ: https://github.com/rebuildup/atcoder-kit
// 出典コミットSHA: c2f98b60fde5c3d4728e32d66c44402450d0ade6
// 出典コード: https://github.com/rebuildup/atcoder-kit/blob/c2f98b60fde5c3d4728e32d66c44402450d0ade6/rust/templates/contest.rs
// 他のファイルからコードをコピーした場合、その出典URLも追加する。

fn read_i() -> i64 {
    let mut b = String::new();
    std::io::stdin().read_line(&mut b).unwrap();
    b.trim().parse::<i64>().unwrap()
}

fn read_vi() -> Vec<i64> {
    let mut b = String::new();
    std::io::stdin().read_line(&mut b).unwrap();
    b.trim()
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}

fn read_s() -> String {
    let mut b = String::new();
    std::io::stdin().read_line(&mut b).unwrap();
    b.trim().to_string()
}

fn main() {}
