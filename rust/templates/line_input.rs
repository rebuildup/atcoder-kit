// 既存スニペット `stdrvi` / `stdrs` を利用した、単純な1行入力の例。
// どちらも1回の呼び出しで1行ずつ読み取る。複数行の配列には不向き。
// 入力例:
// 10 20 30
// hello
// 出力例:
// 60 hello
// 参照URL（必要に応じて）:
// -

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

fn main() {
    let a = read_vi();
    let word = read_s();
    let sum: i64 = a.iter().sum();
    println!("{sum} {word}");
}
