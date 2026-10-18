/// 非負整数 `n` の各桁（10進数）の和を返す。
///
/// 計算量: O(log10(n + 1))（0 のときも正しく 0 を返す）
/// 例: `digit_sum(12345) == 15`
pub fn digit_sum(mut n: u64) -> u64 {
    let mut sum = 0;
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 基本と境界() {
        assert_eq!(digit_sum(0), 0);
        assert_eq!(digit_sum(7), 7);
        assert_eq!(digit_sum(12345), 15);
        assert_eq!(digit_sum(100_000), 1);
        assert_eq!(digit_sum(u64::MAX), 87);
    }
}
