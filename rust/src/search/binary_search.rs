/// 昇順ソート済みスライスで `x` 以上となる最初の位置を返す。
///
/// 全要素が `x` より小さい場合は `a.len()`。
/// 計算量: O(log n)。空配列でも 0 を返す。
pub fn lower_bound<T: Ord>(a: &[T], x: &T) -> usize {
    let mut left = 0;
    let mut right = a.len();
    while left < right {
        let mid = left + (right - left) / 2;
        if a[mid].cmp(x).is_lt() {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left
}

/// 昇順ソート済みスライスで `x` より大きくなる最初の位置を返す。
///
/// `x` より大きい要素がなければ `a.len()`。
/// 計算量: O(log n)。
pub fn upper_bound<T: Ord>(a: &[T], x: &T) -> usize {
    let mut left = 0;
    let mut right = a.len();
    while left < right {
        let mid = left + (right - left) / 2;
        if a[mid].cmp(x).is_le() {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 重複する要素() {
        let a = [1, 2, 2, 2, 4];
        assert_eq!(lower_bound(&a, &2), 1);
        assert_eq!(upper_bound(&a, &2), 4);
        assert_eq!(lower_bound(&a, &3), 4);
        assert_eq!(upper_bound(&a, &3), 4);
        assert_eq!(lower_bound(&a, &0), 0);
        assert_eq!(upper_bound(&a, &5), 5);
    }

    #[test]
    fn 空と単一要素() {
        assert_eq!(lower_bound::<i32>(&[], &5), 0);
        assert_eq!(upper_bound::<i32>(&[], &5), 0);
        assert_eq!(lower_bound(&[10], &10), 0);
        assert_eq!(upper_bound(&[10], &10), 1);
    }
}
