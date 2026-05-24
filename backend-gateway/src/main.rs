use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, Header, EncodingKey};

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Serialize, Deserialize)]
struct SearchRequest {
    pickup_location: String,
    drop_location: String,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to ONDC Vehicle Booking Gateway")
}

#[post("/login")]
async fn login() -> impl Responder {
    let claims = Claims {
        sub: "user123".to_owned(),
        exp: 10000000000,
    };
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref())).unwrap();
    HttpResponse::Ok().json(token)
}

#[post("/search")]
async fn search(request: web::Json<SearchRequest>) -> impl Responder {
    // Stubbed response: In a real ONDC scenario, this would initiate
    // an asynchronous search call to various BPPs.
    let response = serde_json::json!({
        "status": "success",
        "message": format!("Searching for rides from {} to {}", request.pickup_location, request.drop_location),
        "results": [
            { "provider": "BPP_1", "fare": 150.0, "vehicle_type": "Sedan" },
            { "provider": "BPP_2", "fare": 120.0, "vehicle_type": "Hatchback" }
        ]
    });
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(login)
            .service(search)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
