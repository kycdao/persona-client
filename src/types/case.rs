use json_api_client::types::*;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CaseStatus {
    /// The case is ready to be reviewed or actioned on
    Open,
    /// The case has been reviewed and will have an associated resolution
    Resolved,
    /// DEPRECATED - The closed status is being deprecated in favor of resolved.
    Closed,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attributes {
    status: CaseStatus,
    name: String,
    resolution: Option<String>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: DateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub updated_at: Option<DateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub assigned_at: Option<DateTime>,
    pub creator_id: Option<String>,
    pub creator_type: Option<String>, // enum?
    pub assignee_id: Option<String>,
    pub resolver_id: Option<String>,
    pub resolver_type: Option<String>, // enum?
    pub updater_id: Option<String>,
    pub updater_type: Option<String>, // enum?
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Relationships {
    pub case_template: Relation,
    pub case_comments: RelationList,
    pub accounts: RelationList,
    pub inquiries: RelationList,
    pub reports: RelationList,
}

pub type Case = EntityWithRelations<Attributes, Relationships>;
