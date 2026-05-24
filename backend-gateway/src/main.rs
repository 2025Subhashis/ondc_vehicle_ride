mod models;
use models::{Context, BecknRequest, SearchMessage, Intent, Fulfillment, Location, Catalog, Provider, Descriptor, Item, Price, AckMessage, AckStatus, IssueMessage, OnIssueMessage, SearchRequest, Claims, FareRequest, FareResponse, SelectRequest, InitRequest, ConfirmRequest};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::Utc;
use uuid::Uuid;
use ed25519_dalek::{SigningKey, VerifyingKey, Signer, Signature, Verifier};
use base64::{engine::general_purpose, Engine as _};
use std::collections::HashMap;
use tokio::sync::RwLock;
use rand_core::OsRng;
use redis::{AsyncCommands, Client};

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
    let signature = match ed25519_dalek::Signature::from_slice(&sig_bytes) {
        Ok(sig) => sig,
        Err(_) => return false,
    };
    verifying_key.verify(data.as_bytes(), &signature).is_ok()
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

struct CryptoState {
    signing_key: SigningKey,
    // Mock Registry: Subscriber ID -> VerifyingKey
    participant_registry: HashMap<String, VerifyingKey>,
}

struct AppState {
    redis_client: Client,
    crypto: CryptoState,
}

// Verification helper
fn verify_incoming_request(
    body: &str,
    signature: &str,
    subscriber_id: &str,
    registry: &HashMap<String, VerifyingKey>
) -> bool {
    if let Some(verifying_key) = registry.get(subscriber_id) {
        return verify_data(body, signature, verifying_key);
    }
    false
}

#[get("/poll_search")]
async fn poll_search(
    request: web::Query<std::collections::HashMap<String, String>>,
    data: web::Data<AppState>
) -> impl Responder {
    let transaction_id = request.get("transaction_id").unwrap();
    let mut conn = match data.redis_client.get_async_connection().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    
    match conn.get::<_, String>(transaction_id).await {
        Ok(data) => HttpResponse::Ok().body(data),
        Err(_) => HttpResponse::Accepted().json(serde_json::json!({ "status": "pending" })),
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

#[post("/search")]
async fn search(request: web::Json<SearchRequest>, data: web::Data<AppState>) -> impl Responder {
    let context = create_context("search", None);
    
    // Sign the message (mock)
    let message_str = serde_json::json!({"status": "pending"}).to_string();
    let signature = sign_data(&message_str, &data.crypto.signing_key);
    
    // Initialize state in Redis
    let mut conn = match data.redis_client.get_async_connection().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    let _: () = conn.set(&context.transaction_id, message_str).await.unwrap();
    
    println!("Initiating ONDC Search: transaction_id={}", context.transaction_id);
    
    HttpResponse::Ok().append_header(("X-Gateway-Signature", signature)).json(serde_json::json!({
        "message": { "ack": { "status": "ACK" } },
        "context": context
    }))
}

#[post("/on_search")]
async fn on_search(
    request: web::Json<BecknRequest<Catalog>>,
    data: web::Data<AppState>,
    req: actix_web::HttpRequest
) -> impl Responder {
    let signature = req.headers().get("X-Gateway-Signature").and_then(|v| v.to_str().ok()).unwrap_or("");
    let subscriber_id = "bap.gateway.com"; // Mock - normally extracted from context or registry

    if !verify_incoming_request(&serde_json::to_string(&request.message).unwrap(), signature, subscriber_id, &data.crypto.participant_registry) {
        return HttpResponse::Unauthorized().finish();
    }

    println!("Received valid on_search for transaction_id={}", request.context.transaction_id);
    
    let mut conn = match data.redis_client.get_async_connection().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    
    let _: () = conn.set(&request.context.transaction_id, serde_json::json!(request.message).to_string()).await.unwrap();
    
    HttpResponse::Ok().json(serde_json::json!({ "message": { "ack": { "status": "ACK" } } }))
}

#[post("/select")]
async fn select(request: web::Json<SelectRequest>) -> impl Responder {
    let context = create_context("select", Some(request.transaction_id.clone()));
    
    println!("Initiating ONDC Select: transaction_id={}", context.transaction_id);

    HttpResponse::Ok().json(serde_json::json!({
        "message": { "ack": { "status": "ACK" } },
        "context": context
    }))
}

#[post("/on_select")]
async fn on_select() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "status": "implemented soon" }))
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
    HttpResponse::Ok().json(serde_json::json!({ "status": "implemented soon" }))
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
    let mut conn = match data.redis_client.get_async_connection().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    let _: () = conn.set(&tx_id, catalog.to_string()).await.unwrap();
    HttpResponse::Ok().json(serde_json::json!({"status": "callback_simulated"}))
}
#[post("/issue")]
async fn issue(
    request: web::Json<BecknRequest<IssueMessage>>,
    data: web::Data<AppState>,
    req: actix_web::HttpRequest
) -> impl Responder {
    let signature = req.headers().get("X-Gateway-Signature").and_then(|v| v.to_str().ok()).unwrap_or("");
    let subscriber_id = "bap.gateway.com"; // Mock

    if !verify_incoming_request(&serde_json::to_string(&request.message).unwrap(), signature, subscriber_id, &data.crypto.participant_registry) {
        return HttpResponse::Unauthorized().finish();
    }

    let context = request.context.clone();
    let mut conn = match data.redis_client.get_async_connection().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // Persist issue state in Redis
    let _: () = conn.set(&request.message.issue.id, serde_json::json!(request.message.issue).to_string()).await.unwrap();

    println!("Received ONDC Issue: issue_id={}", request.message.issue.id);

    HttpResponse::Ok().json(serde_json::json!({
        "message": { "ack": { "status": "ACK" } },
        "context": context
    }))
}


#[post("/on_issue")]
async fn on_issue(request: web::Json<BecknRequest<OnIssueMessage>>, data: web::Data<AppState>) -> impl Responder {
    println!("Received on_issue for issue_id={}", request.message.issue.id);
    
    let mut conn = match data.redis_client.get_async_connection().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    
    let _: () = conn.set(&request.message.issue.id, serde_json::json!(request.message.issue).to_string()).await.unwrap();
    
    HttpResponse::Ok().json(serde_json::json!({ "message": { "ack": { "status": "ACK" } } }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    println!("Starting server on port: {}", port);

    // Generate mock keys for local testing
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let verifying_key = signing_key.verifying_key();
    
    // Mock registry: add our own verifying key for simulation
    let mut participant_registry = HashMap::new();
    participant_registry.insert("bap.gateway.com".to_string(), verifying_key);

    // Initialize Redis Client
    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
    let redis_client = redis::Client::open(redis_url).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let app_state = web::Data::new(AppState {
        redis_client,
        crypto: CryptoState {
            signing_key,
            participant_registry,
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
            .service(issue)
            .service(on_issue)
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
