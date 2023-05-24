use json_api_client::types::*;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Attributes {
    reference_id: Option<String>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: DateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub updated_at: Option<DateTime>,
}

pub type Account = Entity<Attributes>;
