use json_api_client::types::*;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum InquiryStatus {
    /// The individual started the inquiry
    Created,
    /// The individual submitted a verification within the inquiry
    Pending,
    /// The individual passed all required verifications within the inquiry
    Completed,
    /// Optional status applied to execute custom decisioning logic
    Approved,
    /// Optional status applied to execute custom decisioning logic
    Declined,
    /// The individual did not complete the inquiry within 24 hours
    Expired,
    /// The individual exceeded the allowed number of verification attempts on the inquiry and cannot continue
    Failed,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attributes {
    pub status: InquiryStatus,
    pub reference_id: Option<String>,
    pub note: Option<String>,
    pub tags: Vec<String>,
    pub creator: String,
    pub reviewer_comment: Option<String>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: DateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub started_at: Option<DateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub completed_at: Option<DateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub failed_at: Option<DateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub decisioned_at: Option<DateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub expired_at: Option<DateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub redacted_at: Option<DateTime>,
    pub previous_step_name: Option<String>, // enum? e.g. "verification_selfie", "success"
    pub next_step_name: String,             // enum?
    pub fields: serde_json::Value,          // what is this struct?
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Relationships {
    pub account: Relation,
    pub template: RelationOption,
    pub inquiry_template: RelationOption,
    pub inquiry_template_version: RelationOption,
    pub reviewer: RelationOption,
    pub reports: RelationList,
    pub verifications: RelationList,
    pub sessions: RelationList,
    pub documents: RelationList,
    pub selfies: RelationList,
}

pub type Inquiry = EntityWithRelations<Attributes, Relationships>;
