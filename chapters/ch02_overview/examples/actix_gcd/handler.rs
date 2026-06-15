//! HTTP 路由处理。

use actix_web::{HttpResponse, get, post, web};
use serde::Deserialize;

use ch02_overview::gcd;

/// 表单提交的字段。
#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

/// 首页：一个提交两个数字的表单。
const INDEX_HTML: &str = r#"<!DOCTYPE html>
<html>
<head><title>GCD Calculator</title></head>
<body>
<form action="/gcd" method="post">
  <input type="text" name="n"/>
  <input type="text" name="m"/>
  <button type="submit">Compute GCD</button>
</form>
</body>
</html>"#;

#[get("/")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(INDEX_HTML)
}

#[post("/gcd")]
pub async fn compute_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring.");
    }

    let body = format!(
        "The greatest common divisor of {} and {} is <b>{}</b>",
        form.n,
        form.m,
        gcd(form.n, form.m)
    );
    HttpResponse::Ok().content_type("text/html").body(body)
}
