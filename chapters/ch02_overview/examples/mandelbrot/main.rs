//! 示例 3：多线程并行绘制曼德布洛特集合
//!
//! 运行（release 模式更快）：
//!     cargo run -p ch02_overview --release --example mandelbrot -- mandel.png 4000x3000 -1.20,0.35 -1,0.20

mod image_io;
mod parse;
mod render;

use std::env;
use std::process;
use std::thread::available_parallelism;

use crossbeam::scope;

use parse::{parse_complex, parse_pair};
use render::render;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT", args[0]);
        eprintln!(
            "Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20",
            args[0]
        );
        process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("error parsing upper left corner point");
    let lower_right = parse_complex(&args[4]).expect("error parsing lower right corner point");

    let mut pixels = vec![0u8; bounds.0 * bounds.1];

    // 将像素缓冲区切成水平条带，每个线程负责一条
    let threads = available_parallelism().map(|n| n.get()).unwrap_or(1);
    let rows_per_band = (bounds.1 + threads - 1) / threads; // 向上取整

    let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();

    scope(|s| {
        for (i, band) in bands.into_iter().enumerate() {
            let top = rows_per_band * i;
            let height = band.len() / bounds.0;
            let band_bounds = (bounds.0, height);
            let band_upper_left = render::pixel_to_point(bounds, (0, top), upper_left, lower_right);
            let band_lower_right =
                render::pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);

            s.spawn(move |_| {
                render(band, band_bounds, band_upper_left, band_lower_right);
            });
        }
    })
    .expect("thread panicked while rendering");

    image_io::write_image(&args[1], &pixels, bounds).expect("error writing PNG file");
}
