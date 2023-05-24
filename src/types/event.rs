use json_api_client::types::*;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload<T> {
    pub data: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attributes<T> {
    pub name: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: DateTime,
    //#[serde(with = "time::serde::rfc3339::option")]
    //pub redacted_at: Option<DateTime>,
    pub payload: Payload<T>,
}

pub type Event<T> = Entity<Attributes<T>>;

pub type WebhookData<T> = Response<Event<T>>;
