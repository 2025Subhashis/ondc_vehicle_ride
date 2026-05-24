use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct FareRequest {
    distance: f64,
    time_of_day: String, // Simplified for now
    supply: f64,
    demand: f64,
}

#[derive(Serialize)]
struct FareResponse {
    fare: f64,
}

#[post("/calculate-fare")]
async fn calculate_fare(request: web::Json<FareRequest>) -> impl Responder {
    let base_rate = 50.0;
    let rate_per_km = 10.0;
    
    // Simple surge calculation: (demand / supply)
    let surge_factor = if request.supply > 0.0 {
        (request.demand / request.supply).max(1.0)
    } else {
        2.0 // Default high surge if no supply
    };

    let fare = (base_rate + (request.distance * rate_per_km)) * surge_factor;
    
    HttpResponse::Ok().json(FareResponse { fare })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT").unwrap_or_else(|_| "8081".to_string());
    println!("Starting pricing engine on port: {}", port);
    
    HttpServer::new(|| {
        App::new()
            .service(calculate_fare)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
