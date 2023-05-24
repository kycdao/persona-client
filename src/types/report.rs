use json_api_client::types::*;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ReportStatus {
    /// While the server processes the report, the report is pending
    Pending,
    /// When the server finishes processing the report, the report is ready
    Ready,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SummaryAttributes {
    reference_id: Option<String>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: DateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub updated_at: Option<DateTime>,
    pub status: ReportStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DetailedAttributes {
    pub status: ReportStatus,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: DateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub completed_at: Option<DateTime>,
    #[serde(flatten)]
    pub name: Name,
    pub birthdate: Date,
    #[serde(flatten)]
    pub address: Address,
}

pub type ReportSummary = Entity<SummaryAttributes>;
pub type ReportDetails = Entity<DetailedAttributes>;
