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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IssueMessage {
    pub issue: Issue,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Issue {
    pub id: String,
    pub category: String,
    pub sub_category: String,
    pub status: String,
    pub description: Description,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Description {
    pub short_desc: String,
    pub long_desc: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OnIssueMessage {
    pub issue: IssueResponse,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IssueResponse {
    pub id: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SearchRequest {
    pub pickup_location: String,
    pub drop_location: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FareRequest {
    pub distance: f64,
    pub time_of_day: String,
    pub supply: f64,
    pub demand: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FareResponse {
    pub fare: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SelectRequest {
    pub item_id: String,
    pub provider_id: String,
    pub transaction_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InitRequest {
    pub transaction_id: String,
    pub billing_info: BillingInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BillingInfo {
    pub name: String,
    pub phone: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConfirmRequest {
    pub transaction_id: String,
}
