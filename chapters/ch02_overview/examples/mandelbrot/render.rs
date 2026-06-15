//! 渲染逻辑：迭代复数函数 `z² + c`，判断每个像素属于集合内的深浅。

use num::Complex;

/// 最多迭代 `limit` 次，判断 `c` 是否属于曼德布洛特集合。
/// - 返回 `Some(i)`：`c` 不属于集合，`i` 是逃逸出半径 2 圆的迭代次数。
/// - 返回 `None`：达到上限仍未逃逸，视为属于集合。
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

/// 把图像中的像素坐标映射到复平面上的点。
/// `bounds`：(宽, 高)；`pixel`：(列, 行)。
pub fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        // pixel.1 向下增加，而虚部向上增加，所以用减号
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

/// 将一个矩形区域渲染进 `pixels`（每个字节一个灰度像素）。
pub fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert_eq!(pixels.len(), bounds.0 * bounds.1);
    for row in 0..bounds.1 {
        for col in 0..bounds.0 {
            let point = pixel_to_point(bounds, (col, row), upper_left, lower_right);
            pixels[row * bounds.0 + col] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pixel_to_point_corner() {
        assert_eq!(
            pixel_to_point(
                (100, 200),
                (25, 175),
                Complex { re: -1.0, im: 1.0 },
                Complex { re: 1.0, im: -1.0 }
            ),
            Complex {
                re: -0.5,
                im: -0.75
            }
        );
    }
}
