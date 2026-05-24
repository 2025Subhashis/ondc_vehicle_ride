use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, DateTime};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
struct Context {
    domain: String,
    country: String,
    city: String,
    action: String,
    core_version: String,
    bap_id: String,
    bap_uri: String,
    transaction_id: String,
    message_id: String,
    timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone)]
struct BecknRequest<T> {
    context: Context,
    message: T,
}

#[derive(Serialize, Deserialize, Clone)]
struct Intent {
    fulfillment: Fulfillment,
}

#[derive(Serialize, Deserialize, Clone)]
struct Fulfillment {
    start: Location,
    end: Location,
}

#[derive(Serialize, Deserialize, Clone)]
struct Location {
    gps: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Catalog {
    providers: Vec<Provider>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Provider {
    id: String,
    descriptor: Descriptor,
    items: Vec<Item>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Descriptor {
    name: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Item {
    id: String,
    descriptor: Descriptor,
    price: Price,
}

#[derive(Serialize, Deserialize, Clone)]
struct Price {
    value: String,
    currency: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct SearchRequest {
    pickup_location: String,
    drop_location: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Claims {
    sub: String,
    exp: usize,
}

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

struct AppState {
    transactions: RwLock<HashMap<String, serde_json::Value>>,
}

#[get("/poll_search")]
async fn poll_search(
    request: web::Query<std::collections::HashMap<String, String>>,
    data: web::Data<AppState>
) -> impl Responder {
    let transaction_id = request.get("transaction_id").unwrap();
    let store = data.transactions.read().await;
    
    match store.get(transaction_id) {
        Some(catalog) => HttpResponse::Ok().json(catalog),
        None => HttpResponse::Accepted().json(serde_json::json!({ "status": "pending" })),
    }
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

fn create_context(action: &str, transaction_id: Option<String>) -> Context {
    Context {
        domain: "nic2004:60221".to_string(),
        country: "IND".to_string(),
        city: "std:080".to_string(),
        action: action.to_string(),
        core_version: "1.0.0".to_string(),
        bap_id: "bap.gateway.com".to_string(),
        bap_uri: "http://localhost:8080".to_string(),
        transaction_id: transaction_id.unwrap_or_else(|| Uuid::new_v4().to_string()),
        message_id: Uuid::new_v4().to_string(),
        timestamp: Utc::now(),
    }
}

#[derive(Serialize, Deserialize)]
struct FareRequest {
    distance: f64,
    time_of_day: String,
    supply: f64,
    demand: f64,
}

#[derive(Serialize, Deserialize)]
struct FareResponse {
    fare: f64,
}

#[post("/search")]
async fn search(request: web::Json<SearchRequest>, data: web::Data<AppState>) -> impl Responder {
    let context = create_context("search", None);
    
    // Call Pricing Engine for dynamic fares
    let client = reqwest::Client::new();
    let pricing_url = "http://127.0.0.1:8081/calculate-fare";
    
    let fare_req = FareRequest {
        distance: 10.5,
        time_of_day: "14:00".to_string(),
        supply: 5.0,
        demand: 8.0,
    };

    let fare_res = match client.post(pricing_url).json(&fare_req).send().await {
        Ok(res) => res.json::<FareResponse>().await.map(|r| r.fare).unwrap_or(150.0),
        Err(_) => 150.0,
    };

    // Initialize state as pending
    let mut store = data.transactions.write().await;
    store.insert(context.transaction_id.clone(), serde_json::json!({ "status": "pending" }));
    drop(store);
    
    println!("Initiating ONDC Search: transaction_id={}", context.transaction_id);
    
    HttpResponse::Ok().json(serde_json::json!({
        "message": { "ack": { "status": "ACK" } },
        "context": context
    }))
}

#[post("/on_search")]
async fn on_search(request: web::Json<BecknRequest<Catalog>>, data: web::Data<AppState>) -> impl Responder {
    println!("Received on_search for transaction_id={}", request.context.transaction_id);
    
    let mut store = data.transactions.write().await;
    store.insert(request.context.transaction_id.clone(), serde_json::json!(request.message));
    
    HttpResponse::Ok().json(serde_json::json!({ "message": { "ack": { "status": "ACK" } } }))
}

#[derive(Serialize, Deserialize, Clone)]
struct SelectRequest {
    item_id: String,
    provider_id: String,
    transaction_id: String,
}

#[post("/select")]
async fn select(request: web::Json<SelectRequest>) -> impl Responder {
    let context = create_context("select", Some(request.transaction_id.clone()));
    let message = serde_json::json!({
        "order": {
            "items": [{ "id": request.item_id, "provider_id": request.provider_id }]
        }
    });
    
    println!("Initiating ONDC Select: transaction_id={}", context.transaction_id);

    HttpResponse::Ok().json(serde_json::json!({
        "message": { "ack": { "status": "ACK" } },
        "context": context
    }))
}

#[post("/on_select")]
async fn on_select() -> impl Responder {
    // Implementation for on_select...
    HttpResponse::Ok().json(serde_json::json!({ "status": "implemented soon" }))
}

#[derive(Serialize, Deserialize, Clone)]
struct InitRequest {
    transaction_id: String,
    billing_info: BillingInfo,
}

#[derive(Serialize, Deserialize, Clone)]
struct BillingInfo {
    name: String,
    phone: String,
}

#[post("/init")]
async fn init(request: web::Json<InitRequest>) -> impl Responder {
    let context = create_context("init", Some(request.transaction_id.clone()));
    
    println!("Initiating ONDC Init: transaction_id={}", context.transaction_id);

    HttpResponse::Ok().json(serde_json::json!({
        "message": { "ack": { "status": "ACK" } },
        "context": context
    }))
}

#[post("/on_init")]
async fn on_init() -> impl Responder {
    // Implementation for on_init...
    HttpResponse::Ok().json(serde_json::json!({ "status": "implemented soon" }))
}

#[derive(Serialize, Deserialize, Clone)]
struct ConfirmRequest {
    transaction_id: String,
}

#[post("/confirm")]
async fn confirm(request: web::Json<ConfirmRequest>) -> impl Responder {
    let context = create_context("confirm", Some(request.transaction_id.clone()));
    
    println!("Initiating ONDC Confirm: transaction_id={}", context.transaction_id);

    HttpResponse::Ok().json(serde_json::json!({
        "message": { "ack": { "status": "ACK" } },
        "context": context
    }))
}

#[post("/on_confirm")]
async fn on_confirm() -> impl Responder {
    // Implementation for on_confirm...
    HttpResponse::Ok().json(serde_json::json!({ "status": "implemented soon" }))
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "status": "running" }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        transactions: RwLock::new(HashMap::new()),
    });
    
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_headers(vec![actix_web::http::header::CONTENT_TYPE, actix_web::http::header::AUTHORIZATION])
            .supports_credentials()
            .max_age(3600);
        App::new()
            .app_data(app_state.clone())
            .wrap(cors)
            .service(index)
            .service(login)
            .service(search)
            .service(on_search)
            .service(poll_search)
            .service(select)
            .service(on_select)
            .service(init)
            .service(on_init)
            .service(confirm)
            .service(on_confirm)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
