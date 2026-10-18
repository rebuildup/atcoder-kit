/// 累積和を作る。戻り値の長さは `a.len() + 1`、先頭は 0。
///
/// `prefix[r] - prefix[l]` が半開区間 `[l, r)` の合計。
/// 計算量: O(n)、メモリ: O(n)。合計が `i64` に収まることが前提。
pub fn prefix_sums(a: &[i64]) -> Vec<i64> {
    let mut prefix = Vec::with_capacity(a.len() + 1);
    prefix.push(0);
    for &value in a {
        prefix.push(prefix.last().copied().unwrap() + value);
    }
    prefix
}

/// `[l, r)` の合計を O(1) で求める。
///
/// `prefix` は `prefix_sums` の結果を渡す。`l <= r <= 元の配列長` が必要。
pub fn range_sum(prefix: &[i64], l: usize, r: usize) -> i64 {
    assert!(l <= r && r < prefix.len(), "区間が範囲外です");
    prefix[r] - prefix[l]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 全体と部分区間() {
        let p = prefix_sums(&[3, -2, 5, 1]);
        assert_eq!(p, [0, 3, 1, 6, 7]);
        assert_eq!(range_sum(&p, 0, 4), 7);
        assert_eq!(range_sum(&p, 1, 3), 3);
        assert_eq!(range_sum(&p, 2, 2), 0);
    }

    #[test]
    fn 空配列() {
        let p = prefix_sums(&[]);
        assert_eq!(p, [0]);
        assert_eq!(range_sum(&p, 0, 0), 0);
    }

    #[test]
    #[should_panic]
    fn 不正な区間() {
        range_sum(&prefix_sums(&[1, 2]), 0, 3);
    }
}
