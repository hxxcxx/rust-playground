//! 示例 2：基于 actix-web 的 GCD 计算 Web 服务
//!
//! 运行：`cargo run -p ch02_overview --example actix_gcd`
//! 自定义端口：`$env:PORT=8080; cargo run -p ch02_overview --example actix_gcd`
//! 然后浏览器访问 http://localhost:3311

mod handler;

use std::env;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3311);

    println!("Serving on http://localhost:{port} ...");
    HttpServer::new(|| {
        App::new()
            .service(handler::index)
            .service(handler::compute_gcd)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
