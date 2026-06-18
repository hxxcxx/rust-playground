//! 把像素缓冲区写入 PNG 文件。

use std::fs::File;
use std::io::BufWriter;

use image::ImageEncoder;
use image::codecs::png::PngEncoder;

/// 将 `pixels`（每个字节一个灰度像素，尺寸由 `bounds` 给出）写入 `filename`。
pub fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    let file = File::create(filename)?;
    let writer = BufWriter::new(file);
    let encoder = PngEncoder::new(writer);
    // `?` 会把 image::ImageError 转成 std::io::Error
    encoder
        .write_image(
            pixels,
            bounds.0 as u32,
            bounds.1 as u32,
            image::ExtendedColorType::L8,
        )
        .map_err(|e| std::io::Error::other(e))?;
    Ok(())
}
