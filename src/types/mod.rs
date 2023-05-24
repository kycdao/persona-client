use serde::{Deserialize, Serialize};

pub mod account;
pub mod case;
pub mod document;
pub mod event;
pub mod inquiry;
pub mod report;
pub mod verification;

pub use account::Account;
pub use case::Case;
pub use document::Document;
pub use event::{Event, WebhookData};
pub use inquiry::Inquiry;
pub use report::{ReportDetails, ReportSummary};
pub use verification::Verification;

#[derive(Serialize, Deserialize, Debug)]
pub struct Links {
    pub prev: Option<String>,
    pub next: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response<T> {
    pub data: T,
    pub links: Option<Links>,
    // meta?
    // TODO KYC-325 heterogeneous list!
    //pub included: Vec<EntityWithRelations>
}

pub type EntityId = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct EntityReference {
    pub id: EntityId,
    #[serde(rename = "type")]
    pub entity_type: EntityType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Relation {
    pub data: Box<EntityReference>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RelationOption {
    pub data: Box<Option<EntityReference>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RelationList {
    pub data: Vec<EntityReference>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum EntityType {
    Account,
    Case,
    CaseTemplate,
    Document,
    #[serde(rename = "document/government-id")]
    DocumentGovernmentId,
    Event,
    Inquiry,
    InquiryTemplate,
    InquiryTemplateVersion,
    InquirySession,
    Report,
    Reviewer,
    Selfie,
    #[serde(rename = "selfie/profile-and-center")]
    SelfieProfileAndCenter,
    Session,
    Template,
    Verification,
    #[serde(rename = "verification/database")]
    VerificationDatabase,
    #[serde(rename = "verification/document")]
    VerificationDocument,
    #[serde(rename = "verification/government-id")]
    VerificationGovernmentId,
    #[serde(rename = "verification/phone-number")]
    VerificationPhoneNumber,
    #[serde(rename = "verification/selfie")]
    VerificationSelfie,
    // Legacy verification types, replaced by VerificationGovernmentId
    #[serde(rename = "verification/driver-license")]
    VerificationDriverLicense,
    #[serde(rename = "verification/national-identity-card")]
    VerificationNationalIdentityCard,
    #[serde(rename = "verification/passport")]
    VerificationPassport,
    #[serde(rename = "verification/other-identity-card")]
    VerificationOtherIdentityCard,
    // what else?
}

// TODO KYC-325 we should use internally tagged enums instead
#[derive(Serialize, Deserialize, Debug)]
pub struct EntityWithRelations<T, U> {
    pub id: EntityId,
    #[serde(rename = "type")]
    pub entity_type: EntityType,
    pub attributes: T,
    pub relationships: U,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entity<T> {
    pub id: EntityId,
    #[serde(rename = "type")]
    pub entity_type: EntityType,
    pub attributes: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Name {
    pub name_first: String,
    pub name_middle: Option<String>,
    pub name_last: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    pub address_street_1: String,
    pub address_street_2: Option<String>,
    pub address_city: String,
    pub address_subdivision: String,
    pub address_subdivision_abbr: Option<String>,
    pub address_postal_code: String,
    pub address_postal_code_abbr: Option<String>,
}

// type template
/*
use serde::{Serialize, Deserialize};
use json_api::types::*;

use crate::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Attributes {

}

#[derive(Serialize, Deserialize, Debug)]
pub struct Relationships {

}

pub type NAME = EntityWithRelations<Attributes, Relationships>;

 */
