use serde::{Deserialize, Serialize};
use chrono::{Utc, DateTime};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Context {
    pub domain: String,
    pub country: String,
    pub city: String,
    pub action: String,
    pub core_version: String,
    pub bap_id: String,
    pub bap_uri: String,
    pub transaction_id: String,
    pub message_id: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BecknRequest<T> {
    pub context: Context,
    pub message: T,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SearchMessage {
    pub intent: Intent,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Intent {
    pub fulfillment: Fulfillment,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Fulfillment {
    pub start: Location,
    pub end: Location,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Location {
    pub gps: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Catalog {
    pub providers: Vec<Provider>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Provider {
    pub id: String,
    pub descriptor: Descriptor,
    pub items: Vec<Item>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Descriptor {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Item {
    pub id: String,
    pub descriptor: Descriptor,
    pub price: Price,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Price {
    pub value: String,
    pub currency: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AckMessage {
    pub ack: AckStatus,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AckStatus {
    pub status: String,
}
