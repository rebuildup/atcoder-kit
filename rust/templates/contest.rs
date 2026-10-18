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
