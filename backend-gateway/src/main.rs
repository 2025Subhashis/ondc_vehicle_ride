use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
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

#[post("/search")]
async fn search(request: web::Json<SearchRequest>) -> impl Responder {
    let context = create_context("search", None);
    let message = Intent {
        fulfillment: Fulfillment {
            start: Location { gps: request.pickup_location.clone() },
            end: Location { gps: request.drop_location.clone() },
        },
    };
    
    let beckn_request = BecknRequest {
        context: context.clone(),
        message,
    };

    println!("Initiating ONDC Search: transaction_id={}", context.transaction_id);
    
    // In a real scenario, this would be sent to a BG or BPP
    // For now, we return the synchronous ACK and simulated the async flow
    HttpResponse::Ok().json(serde_json::json!({
        "message": { "ack": { "status": "ACK" } },
        "context": context
    }))
}

#[post("/on_search")]
async fn on_search(request: web::Json<BecknRequest<Catalog>>) -> impl Responder {
    println!("Received on_search for transaction_id={}", request.context.transaction_id);
    // Process catalog results...
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(login)
            .service(search)
            .service(on_search)
            .service(select)
            .service(on_select)
            .service(init)
            .service(on_init)
            .service(confirm)
            .service(on_confirm)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
