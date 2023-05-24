use json_api_client::types::*;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DocumentKind {
    ProofOfEmployment,
    // what else?
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DocumentStatus {
    /// When the individual is first asked to provide a document, the document is initiated
    Initiated,
    /// When the individual submits their document, the server processes the document
    Submitted,
    ///The server is done processing the document
    Processed,
    /// If the server fails to process the document
    Errored,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub filename: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attributes {
    status: DocumentStatus,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: DateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub processed_at: Option<DateTime>,
    // Generic
    kind: Option<DocumentKind>,
    pub files: Option<Vec<File>>,
    // GovernmentId
    pub front_photo: Option<File>,
    pub back_photo: Option<File>,
    pub selfie_photo: Option<File>,
    pub id_class: String,
    pub endorsements: Option<String>,
    pub restrictions: Option<String>,
    pub vehicle_class: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Relationships {
    pub inquiry: Relation,
}

pub type Document = EntityWithRelations<Attributes, Relationships>;
