use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, DateTime};
use uuid::Uuid;
use ed25519_dalek::{SigningKey, VerifyingKey, Signer, Signature, Verifier};
use base64::{engine::general_purpose, Engine as _};

// --- Crypto Helpers ---

fn sign_data(data: &str, signing_key: &SigningKey) -> String {
    let signature = signing_key.sign(data.as_bytes());
    general_purpose::STANDARD.encode(signature.to_bytes())
}

fn verify_data(data: &str, signature_b64: &str, verifying_key: &VerifyingKey) -> bool {
    let sig_bytes = match general_purpose::STANDARD.decode(signature_b64) {
        Ok(bytes) => bytes,
        Err(_) => return false,
    };
    let signature = match Signature::from_slice(&sig_bytes) {
        Ok(sig) => sig,
        Err(_) => return false,
    };
    verifying_key.verify(data.as_bytes(), &signature).is_ok()
}

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

use rand_core::OsRng; // Add this import

struct CryptoState {
    signing_key: SigningKey,
    verifying_key: VerifyingKey,
}

struct AppState {
    transactions: RwLock<HashMap<String, serde_json::Value>>,
    crypto: CryptoState,
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
    
    // ... (pricing logic)
    let fare_res = 150.0;

    let message = serde_json::json!({
        "intent": {
            "fulfillment": {
                "start": { "gps": request.pickup_location },
                "end": { "gps": request.drop_location }
            }
        }
    });
    
    // Sign the message
    let message_str = message.to_string();
    let signature = sign_data(&message_str, &data.crypto.signing_key);
    
    // Initialize state
    let mut store = data.transactions.write().await;
    store.insert(context.transaction_id.clone(), serde_json::json!({ "status": "pending" }));
    drop(store);
    
    println!("Initiating ONDC Search: transaction_id={}", context.transaction_id);
    
    HttpResponse::Ok().append_header(("X-Gateway-Signature", signature)).json(serde_json::json!({
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

#[post("/simulate_on_search/{transaction_id}")]
async fn simulate_on_search(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let tx_id = path.into_inner();
    let catalog = serde_json::json!({
        "providers": [{
            "id": "provider_456",
            "descriptor": { "name": "ONDC Ride Provider" },
            "items": [{
                "id": "item_123",
                "descriptor": { "name": "Standard Cab" },
                "price": { "value": "250.0", "currency": "INR" }
            }]
        }]
    });
    let mut store = data.transactions.write().await;
    store.insert(tx_id, catalog);
    HttpResponse::Ok().json(serde_json::json!({"status": "callback_simulated"}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    println!("Starting server on port: {}", port);

    // Generate mock keys for local testing
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let verifying_key = signing_key.verifying_key();

    let app_state = web::Data::new(AppState {
        transactions: RwLock::new(HashMap::new()),
        crypto: CryptoState {
            signing_key,
            verifying_key,
        },
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
            .service(simulate_on_search)
            .service(poll_search)
            .service(select)
            .service(on_select)
            .service(init)
            .service(on_init)
            .service(confirm)
            .service(on_confirm)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
