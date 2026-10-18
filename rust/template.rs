// 出典リポジトリ: https://github.com/rebuildup/atcoder-kit
// 出典コミットSHA: e5c53989a5ca39b26db59713fbfbe1bb83a04188
// 出典コード: https://github.com/rebuildup/atcoder-kit/blob/e5c53989a5ca39b26db59713fbfbe1bb83a04188/rust/templates/contest.rs
// 他のファイルからコードをコピーした場合、その出典URLも追加する。

// AtCoder提出用テンプレートの原本。編集する場合はここを変更する。
// rust/template.rs はGitHub Actionsが生成するため、直接編集しない。
// 標準ライブラリのみ。普段使う入力関数だけを収録。

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
