//! 命令行参数解析：把 `"400x600"` 或 `"1.0,0.5"` 这样的字符串解析成数值对/复数。

use num::Complex;
use std::str::FromStr;

/// 将字符串解析为坐标对，如 `"400x600"`、`"1.0,0.5"`。
/// `<sep>` 指定分隔符，左右两段必须都能被 `T::from_str` 解析成功。
pub fn parse_pair<T: FromStr>(s: &str, sep: char) -> Option<(T, T)> {
    let idx = s.find(sep)?;
    let left = T::from_str(&s[..idx]).ok()?;
    let right = T::from_str(&s[idx + sep.len_utf8()..]).ok()?;
    Some((left, right))
}

/// 把逗号分隔的一对浮点数解析成 `Complex<f64>`。
pub fn parse_complex(s: &str) -> Option<Complex<f64>> {
    let (re, im) = parse_pair(s, ',')?;
    Some(Complex { re, im })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_pair_ok() {
        assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
        assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
    }

    #[test]
    fn parse_pair_bad() {
        assert_eq!(parse_pair::<i32>("", ','), None);
        assert_eq!(parse_pair::<i32>("10,", ','), None);
        assert_eq!(parse_pair::<i32>(",10", ','), None);
        assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
        assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    }

    #[test]
    fn parse_complex_ok() {
        assert_eq!(
            parse_complex("1.25,-0.0625"),
            Some(Complex {
                re: 1.25,
                im: -0.0625
            })
        );
    }

    #[test]
    fn parse_complex_bad() {
        assert_eq!(parse_complex(",-0.0625"), None);
    }
}
