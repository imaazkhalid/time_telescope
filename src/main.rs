use actix_files as fs;
use actix_web::{
    get, middleware, post,
    web::{Json, Data},
    App, HttpResponse, HttpServer, Responder,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::io;
use sqlx::{Pool, Sqlite};

mod db;

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
    nearest_landmark: Option<db::CelestialObject>,
    travel_time_voyager: String,
}

#[get("/api/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"status": "healthy"}))
}

#[post("/api/calculate")]
async fn calculate_distance(
    req: Json<CalculationRequest>,
    pool: Data<Pool<Sqlite>>,
) -> impl Responder {
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

    // Find nearest landmark
    // We want the object with the smallest absolute difference in distance
    let landmark = sqlx::query_as::<_, db::CelestialObject>(
        "SELECT * FROM celestial_objects 
         ORDER BY ABS(distance_ly - ?) ASC 
         LIMIT 1"
    )
    .bind(light_years)
    .fetch_optional(pool.get_ref())
    .await
    .unwrap_or(None);

    // Calculate travel time at Voyager 1 speed (~17 km/s)
    // 17 km/s = 61,200 km/h
    // Hours = km / 61200
    // Years = Hours / (24 * 365.25)
    let voyager_speed_kmh = 61_200.0;
    let hours_voyager = kilometers / voyager_speed_kmh;
    let years_voyager = hours_voyager / (24.0 * 365.25);

    let travel_time_voyager = if years_voyager < 1.0 {
        format!("{:.1} days", hours_voyager / 24.0)
    } else if years_voyager < 1000.0 {
        format!("{:.1} years", years_voyager)
    } else if years_voyager < 1_000_000.0 {
        format!("{:.1} thousand years", years_voyager / 1000.0)
    } else {
        format!("{:.1} million years", years_voyager / 1_000_000.0)
    };

    HttpResponse::Ok().json(CalculationResponse {
        light_years,
        kilometers,
        miles,
        years_ago: light_years,
        nearest_landmark: landmark,
        travel_time_voyager,
    })
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let pool = db::init_db().await;

    log::info!("Starting Time Telescope server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .wrap(
                middleware::DefaultHeaders::new()
                    .add(("X-Content-Type-Options", "nosniff"))
                    .add(("X-Frame-Options", "DENY"))
                    .add(("X-XSS-Protection", "1; mode=block"))
                    .add(("Content-Security-Policy", "default-src 'self'; style-src 'self' 'unsafe-inline' https://fonts.googleapis.com; font-src https://fonts.gstatic.com; script-src 'self'; img-src 'self' data:;"))
            )
            .service(health_check)
            .service(calculate_distance)
            .service(fs::Files::new("/", "./static").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
