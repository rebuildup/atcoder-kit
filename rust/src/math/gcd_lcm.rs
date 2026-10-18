/// ユークリッドの互除法で最大公約数を求める。
///
/// gcd(0, 0) は便宜上 0 とする。
/// 計算量: O(log min(a, b))（一方が 0 の場合は定数回）
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

/// 最小公倍数を求める。`u64` を超える場合は `None`。
///
/// いずれかが 0 なら `Some(0)`。
pub fn lcm(a: u64, b: u64) -> Option<u64> {
    if a == 0 || b == 0 {
        return Some(0);
    }
    (a / gcd(a, b)).checked_mul(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 最大公約数() {
        assert_eq!(gcd(12, 18), 6);
        assert_eq!(gcd(13, 17), 1);
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(5, 0), 5);
        assert_eq!(gcd(0, 0), 0);
    }

    #[test]
    fn 最小公倍数とオーバーフロー() {
        assert_eq!(lcm(12, 18), Some(36));
        assert_eq!(lcm(0, 9), Some(0));
        assert_eq!(lcm(7, 7), Some(7));
        assert_eq!(lcm(u64::MAX, 2), None);
    }
}
