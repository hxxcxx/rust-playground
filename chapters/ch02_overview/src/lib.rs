//! 第2章共享代码：`gcd` 函数。

/// 用欧几里得算法计算两个非零 `u64` 的最大公约数。
pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0, "gcd: 两个参数都不能为零");
    while m != 0 {
        if m < n {
            std::mem::swap(&mut n, &mut m);
        }
        m %= n;
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coprime() {
        assert_eq!(gcd(14, 15), 1);
    }

    #[test]
    fn common_factor() {
        assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
    }

    #[test]
    #[should_panic]
    fn zero_panics() {
        gcd(0, 10);
    }
}
