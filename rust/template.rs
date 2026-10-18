// 出典リポジトリ: https://github.com/rebuildup/atcoder-kit
// 出典コミットSHA: e79aefe3d644991dc6cfdbd4b696ed675ab22b3f
// 出典コード: https://github.com/rebuildup/atcoder-kit/blob/e79aefe3d644991dc6cfdbd4b696ed675ab22b3f/rust/templates/contest.rs
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
