use actix_files as fs;
use actix_web::{
    get, middleware, post,
    web::Json,
    App, HttpResponse, HttpServer, Responder,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Deserialize)]
struct CalculationRequest {
    target_date: DateTime<Utc>,
}

#[derive(Serialize)]
struct CalculationResponse {
    light_years: f64,
    kilometers: f64,
    miles: f64,
    years_ago: f64,
}

#[get("/api/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"status": "healthy"}))
}

#[post("/api/calculate")]
async fn calculate_distance(req: Json<CalculationRequest>) -> impl Responder {
    let now = Utc::now();
    let target = req.target_date;

    if target > now {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Target date must be in the past."
        }));
    }

    let duration = now.signed_duration_since(target);
    let seconds = duration.num_seconds() as f64;
    
    // 1 light year = distance light travels in 1 Julian year (365.25 days)
    // seconds in 1 Julian year = 365.25 * 24 * 60 * 60 = 31,557,600
    let seconds_in_year = 31_557_600.0;
    let light_years = seconds / seconds_in_year;
    
    // 1 light year ≈ 9.461 × 10^12 kilometers
    let kilometers = light_years * 9_460_730_472_580.8;
    // 1 light year ≈ 5.879 × 10^12 miles
    let miles = light_years * 5_878_625_373_183.6;

    HttpResponse::Ok().json(CalculationResponse {
        light_years,
        kilometers,
        miles,
        years_ago: light_years,
    })
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Starting Time Telescope server at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(
                middleware::DefaultHeaders::new()
                    .add(("X-Content-Type-Options", "nosniff"))
                    .add(("X-Frame-Options", "DENY"))
                    .add(("X-XSS-Protection", "1; mode=block"))
                    .add(("Content-Security-Policy", "default-src 'self'; style-src 'self' https://fonts.googleapis.com; font-src https://fonts.gstatic.com; script-src 'self';"))
            )
            .service(health_check)
            .service(calculate_distance)
            .service(fs::Files::new("/", "./static").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}