use regex::Regex;
use redactr::load_rule_configs;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder, web};

#[get("/")]
async fn index() -> impl Responder {
    let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Redactr</title>
</head>
<body>
    <h1>Redactr</h1>
    <p>Redactr is a microservice that redacts personal identifiable information (PII) from text.</p>
    <p>It is a HTTP API that accepts a JSON payload with a text field and returns a JSON payload with a redacted_text field.</p>
    <p>It is built with Rust and Actix Web.</p>
    <p>Endpoints available:</p>
    <ul>
        <li>POST <a href="/redact">/redact</a></li>
        <li>GET <a href="/health">/health</a></li>
    </ul>
</body>
</html>"#;
    HttpResponse::Ok().body(html)
}

#[post("/redact")]
async fn redact(input_text: web::Json<String>) -> impl Responder {
    let mut rules = load_rule_configs();

    // Apply the rules sequentially
    let mut redacted_text = input_text.to_string();
    for rule in &mut rules {
        let regex = Regex::new(rule.pattern.as_str()).unwrap();
        for captures in regex.captures_iter(&input_text) {
            let matched_text = captures.get(0).unwrap().as_str();
            let redacted_match = rule.on_match(matched_text);
            redacted_text = redacted_text.replace(matched_text, &redacted_match);
        }
    }

    // Return the redacted text
    HttpResponse::Ok().body(redacted_text)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(redact)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
